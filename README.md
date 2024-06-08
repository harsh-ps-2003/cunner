# Cunner
A pluggable blockchain consensus simulation framework written in Rust.

Testing and benchmarking complex consensus algorithms in existing codebases can sometimes be a pain. Cunner is solving this problem by exposing a simple blockchain behind a p2p network where consensus engines can easily be plugged in and out. 

### How?
The server is emiting transactions each N seconds that will be relayed through the network. Each engine implementation will receive those transactions, hence engines can than operate on those transactions according to their implementation. Messages can be quickly bootstrapped and implemented with `protobuf`.

# References
While going through this [research paper](https://pure.tudelft.nl/ws/portalfiles/portal/132697278/Gromit_Benchmarking_the_Performance_and_Scalability_of_Blockchain_Systems.pdf) I got the inspiration to build this project!

### Example
There is a [engine example](https://github.com/harsh-ps-2003/cunner/blob/main/consensus/example-engine.rs) that should cover the idea and get you up to speed.

Also, an Avalanche consensus algorithm with its corresponding engine is implemented for fun!

<!-- ### Todo
- configuration
- blockchain persistance  -->
