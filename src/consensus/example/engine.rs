use crate::consensus::engine::Engine as EngineTrait;
use crate::network::messages::message::{Transaction, Block};
use secp256k1::SecretKey;
use std::sync::{Arc, Mutex};
use tokio::time::Duration;

pub struct Engine {
    block_generation_interval: Duration,
    // private_key: Option<Arc<SecretKey>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    last_block_index: u32, // Added this field to keep track of the last block index
}

impl EngineTrait for Engine {
    fn add_transaction(&self, transaction: Transaction) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction);
    }

    fn get_new_block(&mut self) -> Option<Block> {
        let transactions: Vec<Transaction> = self.transactions.lock().unwrap().drain(..).collect();
        if transactions.is_empty() {
            return None;
        }
        self.last_block_index += 1;
        Some(Block::new_block(self.last_block_index, transactions))
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

    pub async fn run(&self) {
        // do simple thing here!
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
