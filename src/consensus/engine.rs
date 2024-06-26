use crate::network::messages::message::{Block, Transaction};
use std::future::Future;
use std::pin::Pin;
use dyn_clone::DynClone;
use tokio::time::Duration;
use k256::Secp256k1;
use ecdsa::SigningKey;

/// Engine is a trait abstraction for an algorithm agnostic consensus engine.
pub trait Engine: Send + Sync + DynClone {
    /// add_transaction will be called each time the server sees a tx for the
    /// first time.
    /// if the transaction is already seen, it should be ignored
    fn add_transaction(&self, transaction: Transaction);

    /// relay the created block to the p2p network
    fn get_new_block(&mut self) -> Option<Block>;

    /// runs the engine on the transactions
    fn run<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

dyn_clone::clone_trait_object!(Engine);