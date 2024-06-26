# Consensus Algorithms and their corresponding Engines

A consensus algorithm in a blockchain network is responsible for ensuring that all participants (nodes) in the network agree on the state of the blockchain. It does this by validating transactions, assembling them into blocks, and reaching agreement on the order and inclusion of these blocks in the blockchain. Here’s a more detailed look at the steps involved:
Steps Involved in a Consensus Algorithm

    Transaction Validation - the transaction is properly formatted, the sender has sufficient balance, the transaction is signed correctly, there is no double-spending
    Block Creation (Mining/Validation) - valid transactions from the mempool are selected, a block is assembled with the selected transactions, the hash of the previous block, and other metadata (e.g., timestamp, nonce in PoW), and depending on the consensus mechanism, a proof (e.g., PoW) is generated to ensure the block meets certain criteria.
    Block Proposal
    Consensus Mechanism Execution - Different consensus mechanisms have different ways of reaching agreement in the network
    Block Validation
    Block Finalization
    Incentive Distribution - In some consensus mechanisms, managing the distribution of rewards or penalties.
    Chain Synchronization

Various consensus algorithms (PoW,PoS, [PBFT](https://pmg.csail.mit.edu/papers/osdi99.pdf), etc) can be implemented in the form of consensus engines.

Consensus engines are the core components that define the rules for transaction validation and block creation. This means developers can easily plug in different consensus algorithms and see how they perform. Each engine can handle the incoming transactions according to its specific algorithmic rules.

This suggests that the consensus engine is running independently of the peer connections and continues to process transactions and create blocks even when peers disconnect. Thus temporary unavailability of peer is not an issue, the peer will rejoin with the same ID!

Peer 1 :
```
Listening on /ip4/127.0.0.1/tcp/51812
Listening on /ip4/172.23.33.85/tcp/51812
Discovered a new peer: 12D3KooWQ4aEYRrgTZAmxCxSLtdrhjUmxu4GemkRFYdxiBge9qJg
Generated new transaction: Transaction { nonce: 10260799291609783122 }
Sending transaction: Transaction { nonce: 10260799291609783122 }
Attempting to publish transaction to network
Successfully published transaction to network
Added transaction to engine: Transaction { nonce: 10260799291609783122 }
Received transaction: Transaction { nonce: 8358429481592450433 }
Added transaction to engine: Transaction { nonce: 8358429481592450433 }
Peer has gone offline: 12D3KooWQ4aEYRrgTZAmxCxSLtdrhjUmxu4GemkRFYdxiBge9qJg
Received block: Block { header: Some(Header { index: 2, nonce: 5430119731721914590 }), transactions: [Transaction { nonce: 8358429481592450433 }, Transaction { nonce: 10260799291609783122 }] }
Discovered a new peer: 12D3KooWQ4aEYRrgTZAmxCxSLtdrhjUmxu4GemkRFYdxiBge9qJg
Generated new transaction: Transaction { nonce: 18152019857487357912 }
Sending transaction: Transaction { nonce: 18152019857487357912 }
Attempting to publish transaction to network
Successfully published transaction to network
Added transaction to engine: Transaction { nonce: 18152019857487357912 }
Received transaction: Transaction { nonce: 4609187770577634261 }
Added transaction to engine: Transaction { nonce: 4609187770577634261 }
```
Peer 2:
```
Listening on /ip4/127.0.0.1/tcp/51811
Listening on /ip4/172.23.33.85/tcp/51811
Discovered a new peer: 12D3KooWPdLnCxJuCtTw75KMdSTDV2wLoAzYkYDr5kxLa2jnsUnA
Generated new transaction: Transaction { nonce: 8358429481592450433 }
Sending transaction: Transaction { nonce: 8358429481592450433 }
Attempting to publish transaction to network
Successfully published transaction to network
Added transaction to engine: Transaction { nonce: 8358429481592450433 }
Received transaction: Transaction { nonce: 10260799291609783122 }
Added transaction to engine: Transaction { nonce: 10260799291609783122 }
Peer has gone offline: 12D3KooWPdLnCxJuCtTw75KMdSTDV2wLoAzYkYDr5kxLa2jnsUnA
Received block: Block { header: Some(Header { index: 2, nonce: 3213107909006227795 }), transactions: [Transaction { nonce: 10260799291609783122 }, Transaction { nonce: 8358429481592450433 }] }
Discovered a new peer: 12D3KooWPdLnCxJuCtTw75KMdSTDV2wLoAzYkYDr5kxLa2jnsUnA
Generated new transaction: Transaction { nonce: 4609187770577634261 }
Sending transaction: Transaction { nonce: 4609187770577634261 }
Attempting to publish transaction to network
Successfully published transaction to network
Added transaction to engine: Transaction { nonce: 4609187770577634261 }
Received transaction: Transaction { nonce: 18152019857487357912 }
Added transaction to engine: Transaction { nonce: 18152019857487357912 }
```

I got inspiration of implementing consensus algorithms from this [Hyperledger blog post](https://www.hyperledger.org/blog/2019/01/11/floating-the-sawtooth-raft-implementing-a-consensus-algorithm-in-rust)! 

*The Avalanche consensus algorithm is implemented [here](https://github.com/harsh-ps-2003/cunner/tree/main/src/consensus/avalanche)*

*The Paxos consensus algorithm can be implemented as an exercise reffering from [here](https://noghartt.dev/blog/paxos-made-simple-with-rust/)*
