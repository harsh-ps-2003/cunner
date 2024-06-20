# Network

The p2p network is started by the engine?

### TCP 

TCP operates by establishing a connection using a three-way handshake:

    SYN: The client sends a SYN (synchronize) packet to the server to initiate a connection.
    SYN-ACK: The server responds with a SYN-ACK (synchronize-acknowledge) packet.
    ACK: The client sends an ACK (acknowledge) packet back to the server, and the connection is established.

Data is sent in segments, and TCP ensures all segments are received and reassembled in the correct order. If a segment is lost, TCP retransmits it. This mechanism is crucial for applications requiring reliable data transmission.

Peers establishes direct TCP connections between themselves before data transfer. Ensures data integrity and retransmits lost packets. Manages the rate of data transmission between sender and receiver to prevent overwhelming the receiver. Adjusts the rate of data transmission based on network capacity to avoid congestion. Requires manual handling of connection setup, teardown, and error handling.

### Why going with libp2p modular transport layer?

libp2p’s modular transport layer, which supports multiple transport protocols (we use TCP here) and provides built-in NAT traversal and secure communication. libp2p provides built-in peer discovery mechanisms (we use mDNS here) and handles peer management, reducing the complexity of manually managing peers. libp2p’s pubsub system (GossipSub) can be used for efficient message distribution and handling. Built-in support for secure communication using protocols like TLS and Noise. Ensures encrypted and authenticated connections between peers.

Transport Layer (OSI Model Layer 4).
1. Basic Connectivity: 
2. Security: No built-in encryption or authentication. Developers need to implement their own security protocols if needed.
3. Multiplexing: No native support for stream multiplexing. Each connection handles a single stream, which can lead to inefficiencies.
4. Peer Discovery: No built-in mechanisms for discovering peers. Developers need to implement their own peer discovery protocols.
5. NAT Traversal: Limited support for NAT traversal. Requires manual implementation of techniques like hole punching.

Application Layer (abstracted over multiple layers, typically Layer 5 and above in the OSI Model).
1. Transport Abstraction and Agnostic: Can operate over multiple underlying transport protocols, including TCP, WebRTC, QUIC, etc. Defines interfaces for transport protocols, allowing for flexible and interchangeable use.
2. Security: 
3. Multiplexing: Supports stream multiplexing using protocols like Yamux and mplex. Allows multiple streams over a single connection, making it efficient for complex communication patterns.
4. Peer Discovery: Provides various mechanisms for peer discovery, such as mDNS and Kademlia DHT, thus simplifying the process of finding and connecting to peers.
5. NAT Traversal: Includes support for NAT traversal techniques like AutoNAT and Circuit Relay. Facilitates connectivity even when peers are behind NATs or firewalls.
6. Transport Upgrading: Uses a transport upgrader to negotiate security and multiplexing for each connection. Ensures compatibility and flexibility in different network environments.

So basically, using libp2p's TCP transport provides a more robust, secure, and feature-rich foundation for building P2P networks compared to using simple TCP. It abstracts many complexities and offers built-in solutions for common P2P challenges. 
