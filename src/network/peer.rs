use crate::PeerConfig;
use crate::consensus::engine::Engine;
use crate::network::messages::message::{Message, Transaction};
use crate::network::messages::message::message::Payload;
use libp2p::{
    gossipsub, mdns, tcp,
    swarm::{NetworkBehaviour, SwarmEvent},
    yamux,
    noise,
    futures::StreamExt
};
use crate::network::messages::protobuf::{decode_protobuf, encode_protobuf};
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{io, select, time::sleep};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicU64, Ordering};
// use web3::signing;

static TRANSACTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(NetworkBehaviour)]
struct PeerBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

// sets up the libp2p swarm, subscribes to a gossipsub topic, and starts listening for incoming connections.
pub async fn run_peer(
    configuration: PeerConfig,
    mut engine_instance: Arc<Mutex<Option<Box<dyn Engine>>>>,
) -> Result<(), Box<dyn Error>> {

    // creating a multi-producer, single-consumer channel for Transaction types.
    // decouples the receipt of transactions from their processing, which can help manage load and ensure that network operations don't block transaction processing or vice versa.
    let (tx, mut rx) = mpsc::channel(32);

    let swarm = Arc::new(Mutex::new(create_swarm()?));
    let topic = Arc::new(gossipsub::IdentTopic::new("cunner"));
    let listen_address = configuration.tcp_listen_address
        .map(|port| format!("/ip4/0.0.0.0/tcp/{}", port))
        .unwrap_or_else(|| "/ip4/0.0.0.0/tcp/0".to_string());

    swarm.lock().unwrap().behaviour_mut().gossipsub.subscribe(&topic)?;
    // listen on default address if no port is specified
    swarm.lock().unwrap().listen_on(listen_address.parse()?)?;

    let mut discovered_peers = HashSet::new();
    // let mut processed_transactions: HashSet<Transaction> = HashSet::new();

    let engine_future = {
        let swarm_clone = Arc::clone(&swarm);
        let topic_clone = Arc::clone(&topic);
        let engine_clone = Arc::clone(&engine_instance);

        tokio::spawn(async move {
            loop {
                let block = {
                    // Only hold the lock for a short time
                    let mut engine_guard = engine_clone.lock().unwrap();
                    if let Some(engine) = engine_guard.as_mut() {
                        engine.get_new_block()
                    } else {
                        break; // Exit the loop if there's no engine
                    }
                };
    
                if let Some(block) = block {
                    let message = Message {
                        payload: Some(Payload::Block(block)),
                    };
                    let encoded_message = encode_protobuf(&message).expect("Failed to encode message");
                    if let Err(e) = swarm_clone.lock().unwrap().behaviour_mut().gossipsub.publish(topic_clone.as_ref().clone(), encoded_message) {
                        println!("Failed to publish block: {:?}", e);
                    }
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        })
    };

    let mut engine_future = Box::pin(engine_future);

    loop {
        let mut swarm_guard = swarm.lock().unwrap();
        select! {
            event = swarm_guard.select_next_some() => match event {
                SwarmEvent::Behaviour(PeerBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    drop(swarm_guard);
                    for (peer_id, _multiaddr) in list {
                        println!("Discovered a new peer: {peer_id}");
                        discovered_peers.insert(peer_id);
                        swarm.lock().unwrap().behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(PeerBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    drop(swarm_guard);
                    for (peer_id, _multiaddr) in list {
                        println!("Peer has gone offline: {peer_id}");
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
                                Some(Payload::Transaction(transaction)) => {
                                    println!("Received transaction: {:?}", transaction);
                                    // a transaction is received via gossipsub, sent to the channel
                                    tx.send(transaction.clone()).await.unwrap();
                                },
                                Some(Payload::Block(block)) => {
                                    println!("Received block: {:?}", block);
                                    // Process the block with the consensus engine
                                },
                                None => println!("Received message with empty payload"),
                            }
                        },
                        Err(e) => println!("Failed to decode message: {:?}", e),
                    }
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    drop(swarm_guard);
                    println!("Listening on {address}");
                }
                _ => {
                    drop(swarm_guard);
                }
            },
            _ = sleep(Duration::from_secs(5)) => {
                drop(swarm_guard);
                if !discovered_peers.is_empty() {
                    let transaction = new_transaction(configuration.private_key.as_ref());
                    tx.send(transaction.clone()).await.unwrap();

                    let message = Message {
                        payload: Some(Payload::Transaction(transaction)),
                    };
                    
                    match encode_protobuf(&message) {
                        Ok(encoded_message) => {
                            if let Err(e) = swarm.lock().unwrap().behaviour_mut().gossipsub.publish(topic.as_ref().clone(), encoded_message) {
                                println!("Failed to publish transaction: {:?}", e);
                            }
                        },
                        Err(e) => println!("Failed to encode message: {:?}", e),
                    }
                }
            },
            // listens for transactions from the channel
            Some(transaction) = rx.recv() => {
                drop(swarm_guard);
                let mut engine_guard = engine_instance.lock().unwrap();
                if let Some(engine) = engine_guard.as_mut() {
                    engine.add_transaction(transaction);
                }
            },
            result = &mut engine_future => {
                drop(swarm_guard);
                println!("Engine future completed unexpectedly");
                return result.map_err(|e| e.into());
            }
        }
    }

    fn create_swarm() -> Result<libp2p::Swarm<PeerBehaviour>, Box<dyn Error>> {
        let swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new, // transport level protocol security
                yamux::Config::default, // multiple sub-streams over single network connection
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

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(PeerBehaviour { gossipsub, mdns })
            })?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(swarm)
    }
}

fn new_transaction(private_key: Option<&secp256k1::SecretKey>) -> Transaction {
    let mut rng = thread_rng();
    let nonce = TRANSACTION_COUNTER.fetch_add(1, Ordering::SeqCst);
    
    Transaction {
        nonce: nonce + rng.gen::<u64>(), // Combine counter and random number for extra uniqueness
    }

    // sign the transaction with the private key according to the transaction that you have
}