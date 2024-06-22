# Cunner
A pluggable blockchain consensus simulation framework written in Rust.

Testing and benchmarking complex consensus algorithms in existing codebases can sometimes be a pain. Cunner is solving this problem by exposing a simple blockchain behind a peer-to-peer network where consensus engines can easily be plugged in and out. It provides a platform for experimenting with new or modified consensus algorithms without the risk of impacting live environments. The computational and network load on each node can vary significantly depending on the consensus algorithm used. In some consensus models, nodes may have different roles which can lead to uneven load distribution among the nodes. Algorithms requiring frequent communication between nodes (e.g., for voting or proposing blocks) can increase network load. Algorithms with high computational demands (e.g., PoW) increase the processing load on each node, affecting performance and scalability. Some algorithms might require nodes to maintain extensive records or logs, impacting storage resources.


### How?

In the Consenter framework, the consensus engine runs within each peer node. These engines are responsible for handling transactions and blocks that are communicated through the network. 

The engine is emitting transactions each N seconds that will be relayed through the network. Each engine implementation will receive those transactions, hence engines can than operate on those transactions according to their implementation.

The consensus engine will decide on the validity of transactions, add them to the block. 

Messages can be quickly bootstrapped and implemented with `protobuf`.

# References
While going through this [research paper](https://pure.tudelft.nl/ws/portalfiles/portal/132697278/Gromit_Benchmarking_the_Performance_and_Scalability_of_Blockchain_Systems.pdf) I got the inspiration to build this project!

Referred to [Hyperledger Blockchain Performance Metrics](https://8112310.fs1.hubspotusercontent-na1.net/hubfs/8112310/Hyperledger/Printables/HL_Whitepaper_Metrics_PDF_V1.01.pdf) for benchmarking metrics of consensus algorithm.

### Example
There is a [engine example](https://github.com/harsh-ps-2003/cunner/tree/main/src/consensus/example) that should cover the idea and get you up to speed.

Also, an [Avalanche consensus algorithm](https://github.com/harsh-ps-2003/cunner/blob/main/src/consensus/avalanche/avalanche.rs) with its corresponding engine is implemented for fun!

<!-- ### Todo
- configuration
- blockchain persistance  -->
