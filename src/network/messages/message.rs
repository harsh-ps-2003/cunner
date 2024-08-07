// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof = "message::Payload", tags = "5, 6")]
    pub payload: ::core::option::Option<message::Payload>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "5")]
        Transaction(super::Transaction),
        #[prost(message, tag = "6")]
        Block(super::Block),
    }
}
/// Header represents a very simple block header used for simulation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Header {
    /// Index of the block.
    #[prost(uint32, tag = "1")]
    pub index: u32,
    /// Nonce used to prevent hash collisions.
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
}
/// Block represents a very simple Block used for simulation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Head of the block that also will be used for computing its hash.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// List of recorded transactions.
    #[prost(message, repeated, tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
/// Transaction represents a very simple transaction used for simulation.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// Nonce used to prevent hash collisions.
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}
