use ecdsa::SigningKey;
use crate::messages::{Message, Transaction};  // these are generated by `prost`

/// Engine is a trait abstraction for an algorithm agnostic consensus engine.
pub trait Engine {
    /// Configurate will be called on server startup, where the server will pass
    /// its relay channel, which can be used to relay generated blocks and
    /// consensus messages into the network and the private key.
    fn configurate(&self, relay_channel: std::sync::mpsc::Sender<Message>, private_key: SigningKey);
    // takes a sender channel and a signing key

    /// AddTransaction will be called each time the server sees a tx for the
    /// first time.
    fn add_transaction(&self, transaction: Transaction);
    // takes a transaction object directly!
}