use crate::network::messages::message::Message; // Assuming the Message struct is defined in the protos module

/// Peer represents a remote node in the network that may be backed by any libp2p transport.
pub trait Peer {
    /// Sends a message to the peer.
    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;

    /// Disconnects from the peer with an optional error.
    fn disconnect(&self, error: Option<Box<dyn std::error::Error>>);

    /// Returns the endpoint of the peer as a string.
    fn endpoint(&self) -> String;
}