use crate::consensus::engine::Engine as EngineTrait;
use crate::network::messages::message::{Message, Transaction, Block};
use crate::network::messages::message::message::Payload;
use ecdsa::SigningKey;
use k256::Secp256k1;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::Duration;

pub struct Engine {
    block_generation_interval: Duration,
    private_key: Option<Arc<SigningKey<Secp256k1>>>,
    relay_channel: Option<mpsc::Sender<Message>>,
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
            private_key: None,
            relay_channel: None,
            transactions: Arc::new(Mutex::new(vec![])),
            last_block_index: 0,
        })
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(self.block_generation_interval);
        loop {
            interval.tick().await;
            let transactions = self.transactions.lock().unwrap().clone();
            let block = Block::new_block(self.last_block_index, transactions);
            if let Some(relay_channel) = &self.relay_channel {
                let message = Message {
                    payload: Some(Payload::Block(block)),
                };
                relay_channel.send(message).await.unwrap();
            }
            self.transactions.lock().unwrap().clear();
        }
    }

    // Additional methods that were in the original implementation
    pub fn set_private_key(&mut self, private_key: Arc<SigningKey<Secp256k1>>) {
        self.private_key = Some(private_key);
    }

    pub fn set_relay_channel(&mut self, relay_channel: mpsc::Sender<Message>) {
        self.relay_channel = Some(relay_channel);
    }
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Self {
            block_generation_interval: self.block_generation_interval,
            private_key: self.private_key.clone(),
            relay_channel: self.relay_channel.clone(),
            transactions: self.transactions.clone(),
            last_block_index: self.last_block_index,
        }
    }
}