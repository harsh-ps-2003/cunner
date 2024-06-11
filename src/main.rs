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
    pub mod server;
    pub mod messages {
        pub mod protobuf;
        pub mod message;
        pub mod messages;
    }
}

mod storage {
    pub mod store;
}

use clap::{Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use secp256k1::Secp256k1;

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
        #[arg(long, help = "Seed nodes to connect to")]
        seed: Option<String>,
        #[arg(long, help = "Enable consensus mode")]
        consensus: bool,
        #[arg(long, help = "Private key for the node")]
        private_key: Option<String>,
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Node { tcp, seed, consensus, private_key, engine } => {
            start_server(tcp, seed, consensus, private_key, engine)?;
        }
    }

    Ok(())
}

fn start_server(tcp: Option<u16>, seed: Option<String>, consensus: bool, private_key: Option<String>, engine: Option<DefinedEngines>) -> Result<(), Box<dyn Error>> {
    let mut engine_instance: Option<Box<dyn consensus::Engine>> = None;

    if consensus {
        // why private key twice?
        let private_key = private_key.ok_or("missing private key for consensus node")?;
        let secp = Secp256k1::new();
        let private_key = secp256k1::SecretKey::from_str(&private_key)?;

        match engine {
            Some(DefinedEngines::Solo) => {
                engine_instance = Some(Box::new(solo::Engine::new_engine(Duration::from_secs(15))));
            },
            Some(DefinedEngines::Avalanche) => {
                engine_instance = Some(Box::new(avalanche::Engine::new_engine(Duration::from_secs(15))));
            },
            // add more of your own!
            None => return Err("engine cannot be empty if running a consensus node".into()),
            _ => return Err("invalid engine option".into()),
        }
    }

    let configuration = network::ServerConfig {
        listen_addr: tcp.unwrap_or(0),
        dial_timeout: Duration::from_secs(3),
        bootstrap_nodes: parse_seeds(seed.as_deref()),
        consensus,
        private_key: Some(private_key),
    };

    let server = network::Server::new(configuration, engine_instance);
    server.start()?;

    Ok(())
}

fn parse_seeds(seeds: Option<&str>) -> Vec<String> {
    seeds
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_default()
}