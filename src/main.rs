mod consensus {
    pub mod example {
        pub mod solo_engine;
    }
    pub mod engine;
    pub mod avalanche {
        pub mod engine;
    }
}

mod network {
    pub mod peer;
    pub mod messages {
        pub mod protobuf;
        pub mod message; // generated by protobuf
        pub mod messages;
    }
}

mod storage {
    pub mod store;
}

use clap::{Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::time::Duration;
use tracing_subscriber::EnvFilter;
use network::peer::run_peer;
use std::sync::{Arc, Mutex};

#[derive(Parser)]
#[command(about = "Pluggable blockchain consensus simulation framework", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// how will the commands work, how will multiple nodes work!
#[derive(Subcommand)]
enum Commands {
    /// Start a single cunner node
    Node {
        #[arg(long, help = "TCP address to bind to")]
        tcp: Option<u16>,
        #[arg(long, help = "Private key for the node who is creating the transaction")]
        private_key: Option<secp256k1::SecretKey>,
        #[arg(long, help = "Consensus engine to use")]
        engine: Option<DefinedEngines>,
    },
}

#[derive(Clone, ValueEnum)]
enum DefinedEngines {
    Solo,
    Avalanche,
    // add more of your own!
}

pub struct PeerConfig {
    tcp_listen_address: Option<u16>,
    private_key: Option<secp256k1::SecretKey>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Node {
            tcp,
            private_key,
            engine,
        } => {
            start_peer(tcp, private_key, engine)?;
        }
    }

    Ok(())
}

// initializes the consensus engine based on the provided option, sets up the peer configuration, and starts the network operations.
fn start_peer(
    tcp: Option<u16>,
    private_key: Option<secp256k1::SecretKey>,
    engine: Option<DefinedEngines>,
) -> Result<(), Box<dyn Error>> {
    let mut engine_instance: Arc<Mutex<Option<Box<dyn consensus::engine::Engine>>>> = Arc::new(Mutex::new(None));

    let private_key = private_key.ok_or("missing private key for consensus node")?;

        match engine {
            Some(DefinedEngines::Solo) => {
                engine_instance = Arc::new(Mutex::new(Some(consensus::example::solo_engine::Engine::new_engine(Duration::from_secs(15)))));
            }
            Some(DefinedEngines::Avalanche) => {
                engine_instance = Arc::new(Mutex::new(Some(consensus::avalanche::engine::Engine::new_engine(Duration::from_secs(15)))));
            }
            // add more of your own!
            None => return Err("engine cannot be empty if running a consensus node".into()),
            _ => return Err("invalid engine option".into()),
    }

    let peer_configuration = PeerConfig {
        tcp_listen_address: Some(tcp.unwrap_or(0)),
        private_key: Some(private_key),
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    run_peer(peer_configuration, engine_instance);

    Ok(())
}

