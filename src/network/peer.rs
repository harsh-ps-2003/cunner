use crate::consensus::engine::Engine;
use crate::network::messages::message::message::Payload;
use crate::network::messages::message::Block;
use crate::network::messages::message::{Message, Transaction};
use crate::network::messages::protobuf::{decode_protobuf, encode_protobuf};
use crate::CunnerError;
use crate::PeerConfig;
use libp2p::Swarm;
use libp2p::{
    futures::StreamExt,
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId,
};
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::error::Error as StdError;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::{io, select, time::sleep};
// use web3::signing;

static TRANSACTION_COUNTER: AtomicU64 = AtomicU64::new(0);
static NETWORK_CONTEXT: Lazy<
    Mutex<Option<(Arc<Mutex<Swarm<PeerBehaviour>>>, Arc<gossipsub::IdentTopic>)>>,
> = Lazy::new(|| Mutex::new(None));

#[derive(NetworkBehaviour)]
struct PeerBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

// sets up the libp2p swarm, subscribes to a gossipsub topic, and starts listening for incoming connections
pub async fn run_peer(
    configuration: PeerConfig,
    engine_instance: Arc<Mutex<Option<Box<dyn Engine>>>>,
) -> Result<(), CunnerError> {
    // creating a multi-producer, single-consumer channel for Transaction types.
    // decouples the receipt of transactions from their processing, which can help manage load and ensure that network operations don't block transaction processing or vice versa.
    let (tx, mut rx) = mpsc::channel(32);

    // creates a new libp2p swarm with the provided configuration and custom behaviour
    let swarm = Arc::new(Mutex::new(
        create_swarm().map_err(|e| CunnerError::Network(e.to_string()))?,
    ));
    let topic = Arc::new(gossipsub::IdentTopic::new("cunner"));

    // stores the swarm and topic in a lazy-initialized mutex for thread-safe access
    *NETWORK_CONTEXT.lock().unwrap() = Some((swarm.clone(), topic.clone()));

    let listen_address = configuration
        .tcp_listen_address
        .map(|port| format!("/ip4/0.0.0.0/tcp/{}", port))
        .unwrap_or_else(|| "/ip4/0.0.0.0/tcp/0".to_string());

    swarm
        .lock()
        .unwrap()
        .behaviour_mut()
        .gossipsub
        .subscribe(&topic)
        .map_err(|e| CunnerError::Network(format!("Failed to subscribe to topic: {}", e)))?;

    // listen on default address if no port is specified
    swarm
        .lock()
        .unwrap()
        .listen_on(
            listen_address
                .parse()
                .map_err(|e| CunnerError::Network(format!("Invalid listen address: {}", e)))?,
        )
        .map_err(|e| CunnerError::Network(format!("Failed to listen on address: {}", e)))?;

    // a set to keep track of discovered peers
    let mut discovered_peers = HashSet::new();
    // let mut processed_transactions: HashSet<Transaction> = HashSet::new();

    let _engine_run_future = {
        let engine_clone = Arc::clone(&engine_instance);
        tokio::spawn(async move {
            loop {
                let engine = {
                    let guard = engine_clone.lock().unwrap();
                    guard.as_ref().cloned()
                };
                if let Some(engine) = engine {
                    tokio::select! {
                        _ = engine.run() => {},
                        _ = tokio::time::sleep(Duration::from_secs(60)) => {
                            warn!("Engine run timed out, restarting...");
                        }
                    }
                } else {
                    break;
                }
            }
        })
    };

    loop {
        let mut swarm_guard = swarm.lock().unwrap();
        select! {
            event = swarm_guard.select_next_some() => match event {
                SwarmEvent::Behaviour(PeerBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    drop(swarm_guard);
                    for (peer_id, _multiaddr) in list {
                        info!("Discovered a new peer: {peer_id}");
                        discovered_peers.insert(peer_id);
                        swarm.lock().unwrap().behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(PeerBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    drop(swarm_guard);
                    for (peer_id, _multiaddr) in list {
                        warn!("Peer has gone offline: {peer_id}");
                        discovered_peers.remove(&peer_id);
                        swarm.lock().unwrap().behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(PeerBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    message_id: _,
                    message,
                    ..
                })) => {
                    drop(swarm_guard);
                    match decode_protobuf(&message.data) {
                        Ok(decoded_message) => {
                            match decoded_message.payload {
                                // a transaction is received via gossipsub, sent to the channel
                                Some(Payload::Transaction(transaction)) => {
                                    debug!("Received transaction: {:?}", transaction);
                                    tx.send(transaction.clone()).await.map_err(|e| CunnerError::Network(format!("Failed to send transaction: {}", e)))?;
                                },
                                // Process the block with the consensus engine
                                Some(Payload::Block(block)) => {
                                    info!("Received block: {:?}", block);
                                },
                                None => warn!("Received message with empty payload"),
                            }
                        },
                        Err(e) => error!("Failed to decode message: {:?}", e),
                    }
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    drop(swarm_guard);
                    info!("Listening on {address}");
                }
                _ => {
                    drop(swarm_guard);
                }
            },
            _ = sleep(Duration::from_secs(5)) => {
                drop(swarm_guard);
                if !discovered_peers.is_empty() {
                    emit_transaction(tx.clone(), swarm.clone(), topic.clone(), &discovered_peers).await;
                }
            },

            // listens for transactions from the channel
            Some(transaction) = rx.recv() => {
                drop(swarm_guard);
                let mut engine_guard = engine_instance.lock().unwrap();
                if let Some(engine) = engine_guard.as_mut() {
                    engine.add_transaction(transaction.clone());
                    debug!("Added transaction to engine: {:?}", transaction.clone());
                }
            }
        }
    }
}

fn create_swarm() -> Result<libp2p::Swarm<PeerBehaviour>, Box<dyn StdError>> {
    let swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(PeerBehaviour { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    Ok(swarm)
}

async fn emit_transaction(
    tx: mpsc::Sender<Transaction>,
    swarm: Arc<Mutex<libp2p::Swarm<PeerBehaviour>>>,
    topic: Arc<gossipsub::IdentTopic>,
    discovered_peers: &HashSet<PeerId>,
) {
    if discovered_peers.is_empty() {
        warn!("No peers discovered, skipping transaction emission");
        return;
    }

    let transaction = new_transaction();
    debug!("Generated new transaction: {:?}", transaction);
    tx.send(transaction.clone()).await.unwrap();
    debug!("Sending transaction: {:?}", transaction);
    let message = Message {
        payload: Some(Payload::Transaction(transaction)),
    };
    info!("Attempting to publish transaction to network");
    match encode_protobuf(&message) {
        Ok(encoded_message) => {
            if let Err(e) = swarm
                .lock()
                .unwrap()
                .behaviour_mut()
                .gossipsub
                .publish(topic.as_ref().clone(), encoded_message)
            {
                error!("Failed to publish transaction: {:?}", e);
            } else {
                info!("Successfully published transaction to network");
            }
        }
        Err(e) => error!("Failed to encode message: {:?}", e),
    }
}

pub fn publish_block(block: Block) {
    let message = Message {
        payload: Some(Payload::Block(block)),
    };
    let encoded_message = encode_protobuf(&message).expect("Failed to encode message");

    if let Some((swarm, topic)) = NETWORK_CONTEXT.lock().unwrap().as_ref() {
        if let Err(e) = swarm
            .lock()
            .unwrap()
            .behaviour_mut()
            .gossipsub
            .publish(topic.as_ref().clone(), encoded_message)
        {
            error!("Failed to publish block: {:?}", e);
        } else {
            info!("Successfully published block to network");
        }
    } else {
        error!("Network context not initialized");
    }
}

fn new_transaction() -> Transaction {
    let mut rng = thread_rng();
    let nonce = TRANSACTION_COUNTER.fetch_add(1, Ordering::SeqCst);

    Transaction {
        nonce: nonce + rng.gen::<u64>(), // unique nonce
    }

    // sign the transaction with the private key according to the transaction that you have
}
