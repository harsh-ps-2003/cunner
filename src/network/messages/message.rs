// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration = "Flag", tag = "1")]
    pub flag: i32,
    #[prost(oneof = "message::Payload", tags = "2, 3, 4, 5, 6")]
    pub payload: ::core::option::Option<message::Payload>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "2")]
        State(super::State),
        #[prost(message, tag = "3")]
        PeerRequest(super::PeerRequest),
        #[prost(message, tag = "4")]
        PeerResponse(super::PeerResponse),
        #[prost(message, tag = "5")]
        Transaction(super::Transaction),
        #[prost(message, tag = "6")]
        Block(super::Block),
    }
}
/// State is used in the initial handshake.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    /// unique peer identifier.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// port the peer is accepting connections on.
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
/// PeerRequest requests known peers in the network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerRequest {
    /// A list of already known peers in the network.
    #[prost(string, repeated, tag = "1")]
    pub known: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PeerResponse contains a list of known peers responded after a PeerRequest is
/// send.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerResponse {
    #[prost(message, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<Peer>,
}
/// Peer holds information about a peer in the network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(string, tag = "1")]
    pub enpoint: ::prost::alloc::string::String,
}
/// Header represents a very simple block header used for simulation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// Nonce used to prevent hash collisions.
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Flag {
    Consensus = 0,
    Payload = 1,
}
impl Flag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Flag::Consensus => "consensus",
            Flag::Payload => "payload",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "consensus" => Some(Self::Consensus),
            "payload" => Some(Self::Payload),
            _ => None,
        }
    }
}
