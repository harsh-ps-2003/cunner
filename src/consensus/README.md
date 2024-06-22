# Consensus Algorithms and their corresponding Engines

A consensus algorithm in a blockchain network is responsible for ensuring that all participants (nodes) in the network agree on the state of the blockchain. It does this by validating transactions, assembling them into blocks, and reaching agreement on the order and inclusion of these blocks in the blockchain. Here’s a more detailed look at the steps involved:
Steps Involved in a Consensus Algorithm

    Transaction Validation - the transaction is properly formatted, the sender has sufficient balance, the transaction is signed correctly, there is no double-spending
    Block Creation (Mining/Validation) - valid transactions from the mempool are selected, a block is assembled with the selected transactions, the hash of the previous block, and other metadata (e.g., timestamp, nonce in PoW), and depending on the consensus mechanism, a proof (e.g., PoW) is generated to ensure the block meets certain criteria.
    Block Proposal
    Consensus Mechanism Execution - Different consensus mechanisms have different ways of reaching agreement in the network
    Block Validation
    Block Finalization
    Chain Synchronization

Various consensus algorithms (PoW,PoS, [PBFT](https://pmg.csail.mit.edu/papers/osdi99.pdf), etc) can be implemented in the form of consensus engines.

Consensus engines are the core components that define the rules for transaction validation and block creation. This means developers can easily plug in different consensus algorithms and see how they perform. Each engine can handle the incoming transactions according to its specific algorithmic rules.

I got inspiration of implementing consensus algorithms from this [Hyperledger blog post](https://www.hyperledger.org/blog/2019/01/11/floating-the-sawtooth-raft-implementing-a-consensus-algorithm-in-rust)! 

*The Avalanche consensus algorithm is implemented [here](https://github.com/harsh-ps-2003/cunner/tree/main/src/consensus/avalanche)*

The Paxos consensus algorithm can be implemented as an exercise reffering from [here](https://noghartt.dev/blog/paxos-made-simple-with-rust/)