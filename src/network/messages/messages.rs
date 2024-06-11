use prost::Message;
use rand::Rng;
use sha2::{Sha256, Digest};
use std::sync::Once;
use bytes::Bytes;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        let mut rng = rand::thread_rng();
        rng.gen::<u64>(); // Seed the RNG
    });
}

#[derive(Message)]
pub struct Block {
    #[prost(message, required, tag = "1")]
    pub header: Header,
}

#[derive(Message)]
pub struct Header {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
}

#[derive(Message)]
pub struct Transaction {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}

impl Block {
    pub fn new_block(prev_index: u32) -> Block {
        init();
        let mut rng = rand::thread_rng();
        Block {
            header: Header {
                index: prev_index + 1,
                nonce: rng.gen(),
            },
        }
    }
}

impl Transaction {
    pub fn new_transaction() -> Transaction {
        init();
        let mut rng = rand::thread_rng();
        Transaction {
            nonce: rng.gen(),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode(&mut buf).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&buf);
        let result = hasher.finalize();
        result.to_vec()
    }
}