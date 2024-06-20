use crate::network::messages::message::{Transaction, Block, Header};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::sync::Once;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        let mut rng = rand::thread_rng();
        rng.gen::<u64>(); // Seed the RNG
    });
}

impl Block {
    pub fn new_block(prev_index: u32, transactions: Vec<Transaction>) -> Block {
        init();
        let mut rng = rand::thread_rng();
        Block {
            header: Some(Header {
                index: prev_index + 1,
                nonce: rng.gen(),
            }),
            transactions,
        }
    }
}

impl Transaction {
    pub fn new_transaction() -> Transaction {
        init();
        let mut rng = rand::thread_rng();
        Transaction { nonce: rng.gen() }
    }

    pub fn hash(&self) -> Vec<u8> {
        let serialized = serde_json::to_vec(&self).expect("Failed to serialize transaction");
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        let result = hasher.finalize();
        let mut hasher = Sha256::new();
        hasher.update(result);
        let double_hashed = hasher.finalize();

        double_hashed.to_vec()
    }
}
