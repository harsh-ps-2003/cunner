use crate::consensus::engine::Engine as EngineTrait;
use crate::network::messages::message::{Block, Transaction};
use crate::network::peer::publish_block;
// use secp256k1::SecretKey;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::time::Duration;

pub struct Engine {
    block_generation_interval: Duration,
    // private_key: Option<Arc<SecretKey>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    last_block_index: u32, // Added this field to keep track of the last block index
}

impl EngineTrait for Engine {
    // process the transactions by the engine
    fn run<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            println!("Engine is processing transactions");
            loop {
                tokio::time::sleep(self.block_generation_interval).await;

                let new_block = {
                    let mut transactions = self.transactions.lock().unwrap();
                    if transactions.is_empty() {
                        println!("No transactions to process");
                        continue;
                    }

                    for transaction in transactions.iter() {
                        println!("Processing transaction: {:?}", transaction);
                    }

                    let block_transactions = transactions.drain(..).collect();
                    Block::new_block(self.last_block_index + 1, block_transactions)
                };

                println!("Created new block: {:?}", new_block);
                publish_block(new_block);
            }
        })
    }

    fn add_transaction(&self, transaction: Transaction) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction);
    }
}

impl Engine {
    pub fn new_engine(interval: Duration) -> Box<dyn EngineTrait> {
        Box::new(Self {
            block_generation_interval: interval,
            // private_key: private_key,
            transactions: Arc::new(Mutex::new(vec![])),
            last_block_index: 0,
        })
    }
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Self {
            block_generation_interval: self.block_generation_interval,
            // private_key: self.private_key.clone(),
            transactions: self.transactions.clone(),
            last_block_index: self.last_block_index,
        }
    }
}
