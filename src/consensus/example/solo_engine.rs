/*
Serving as an example of how to create a custom consensus engine to use the Cunner framework.

The solo engine is used as follows:
    1. Server is configured to use the solo engine:
    ```
        engine := solo.new_engine(time.Second * 5)
        server := network.new_server(configuration, engine)
    ```
    2. On startup, the server calls engine.configurate, passing the relay channel and private key.
    3. The server calls engine.add_transaction when it receives a new transaction message.
    4. The solo engine periodically proposes blocks and sends them to the server via the relay channel.
    5. The server broadcasts the block messages to connected peers.
*/

use crate::network::messages::message::{Message, Transaction};
use crate::network::messages::messages::Block;
use ecdsa::SigningKey;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::{self, Instant};

/// Engine represents a single consensus engine. This is used as an implementation example.
pub struct Engine {
    block_generation_interval: Duration, // Time between block proposals
    private_key: Option<Arc<SigningKey>>, // Private key for signing blocks
    relay_channel: Option<mpsc::Sender<Message>>, // Channel for relaying messages (blocks) to the server
    transactions: Arc<Mutex<Vec<Transaction>>>,   // Buffer for pending transactions
}

impl Engine {
    /// Returns a new solo consensus engine.
    pub fn new_engine(interval: Duration) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            block_generation_interval: interval,
            private_key: None,
            relay_channel: None,
            transactions: Arc::new(Mutex::new(vec![])),
        }))
    }

    /// Configures the engine with a relay channel and a private key, called by server on startup!
    pub fn configurate(
        &mut self,
        relay_channel: mpsc::Sender<Message>,
        private_key: Arc<SigningKey>,
    ) {
        self.private_key = Some(private_key);
        self.relay_channel = Some(relay_channel);
        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.run().await;
        });
    }

    /// The main loop that runs the engine, generating blocks at intervals.
    async fn run(&self) {
        // create a new interval that will be used to generate blocks at the specified interval
        let mut interval = time::interval(self.block_generation_interval);
        // the index of the block
        let mut index: u32 = 0;

        loop {
            interval.tick().await;
            // get a copy of the transactions buffer
            let transactions = self.transactions.lock().unwrap().clone();
            // create a new block with the transactions
            let block = Block::new_block(index);
            block.transactions = transactions;
            // if the relay channel is set, send the block to the server
            if let Some(ref relay_channel) = self.relay_channel {
                relay_channel
                    .send(Message::from_block(block))
                    .await
                    .unwrap();
            }
            // increment the index
            index += 1;
            // clear the transactions buffer
            self.transactions.lock().unwrap().clear();
        }
    }

    /// Adds a transaction to the engine.
    /// Adds a new transaction to the pending buffer. Called by the server when it receives a transaction message.
    pub fn add_transaction(&self, tx: Transaction) {
        // get a mutable reference to the transactions buffer
        let mut transactions = self.transactions.lock().unwrap();
        // add the transaction to the buffer
        transactions.push(tx);
    }
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Self {
            block_generation_interval: self.block_generation_interval,
            private_key: self.private_key.clone(),
            relay_channel: self.relay_channel.clone(),
            transactions: self.transactions.clone(),
        }
    }
}
