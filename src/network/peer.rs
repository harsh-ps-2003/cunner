use futures::stream::StreamExt;
use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux};
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::error::Error;
use std::hash::{Hash, Hasher};
use tokio::{io, select, time::sleep};
use std::time::Duration;
use crate::{PeerConfig, DefinedEngines};

#[derive(NetworkBehaviour)]
struct PeerBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

// sets up the libp2p swarm, subscribes to a gossipsub topic, and starts listening for incoming connections.
pub async fn run_peer(configuration: PeerConfig, engine_instance: Option<Box<dyn Engine>>) -> Result<(), Box<dyn Error>> {
    let mut swarm = create_swarm()?;
    let topic = gossipsub::IdentTopic::new("test-net");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // if tcp address to listen is given, then use that otherwise use the default one!
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut discovered_peers = HashSet::new();
    let mut processed_messages = HashSet::new();

    loop {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(PeerBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("Discovered a new peer: {peer_id}");
                        discovered_peers.insert(peer_id);
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(PeerBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("Peer has gone offline: {peer_id}");
                        discovered_peers.remove(&peer_id);
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(PeerBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    message_id,
                    message,
                    ..
                })) => {
                    let msg_str = String::from_utf8_lossy(&message.data);
                    println!("Received message: '{}'", msg_str);

                    // When a message is received, it's processed by the consensus engine and then relayed to other peers using the gossipsub protocol.
                    if !processed_messages.contains(&message_id) {
                        processed_messages.insert(message_id);
                        // consensus engine has to process message
                        consensus_engine.process_message(&msg_str);

                        // Relay the message to other peers
                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.data) {
                            println!("Relay error: {e:?}");
                        }
                    }
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address}");
                }
                _ => {}
            },
            _ = sleep(Duration::from_secs(1)) => {
                if !discovered_peers.is_empty() {
                    let message = format!("Message from peer: {}", ); // message about the transaction 
                    if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.as_bytes()) {
                        match e {
                            gossipsub::PublishError::InsufficientPeers => {
                                println!("Insufficient peers to publish. Please open another terminal and run the binary to add more peers.");
                            },
                            _ => println!("Publish error: {e:?}"),
                        }
                    }
                }
        }
    }
}

fn create_swarm() -> Result<libp2p::Swarm<PeerBehaviour>, Box<dyn Error>> {
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

            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(PeerBehaviour { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    Ok(swarm)
}