# Avalanche
This is a research implementation of the Avalanche consensus.

Please note that this implementation is not a part of the Cunner framework.
It is a standalone implementation that can be used as a starting point for
building a custom consensus engine.


### Research Papers
The implementation is inspired by the following :

1. [Snowflake to Avalanche: A Novel Metastable Consensus Protocol Family for
Cryptocurrencies](https://ipfs.io/ipfs/QmUy4jh5mGNZvLkjies1RWM4YuvJh5o2FYopNPVYwrRVGV)
2. [Scalable and Probabilistic Leaderless BFT Consensus through Metastability](https://assets-global.website-files.com/5d80307810123f5ffbb34d6e/6009805681b416f34dcae012_Avalanche%20Consensus%20Whitepaper.pdf)


### Context
The first applied solution to [Byzantine Generals Problem](https://lamport.azurewebsites.net/pubs/byz.pdf) was a Classical Consensus. One big advantage of classical consensus protocols is that they achieve finality (the point at which a state cannot be altered or reversed) extremely quickly. However, a major disadvantage of these systems is that all of the nodes in the network need to know one another. This means that the system must be both relatively small, and permissioned. This also means that classical consensus systems are not scalable, as scalability requires the number of nodes to proliferate to the point that it is impossible for each node to be known. Because of this, whilst extremely useful in certain private instances, classical consensus could not support a scalable system such as is required in a cryptocurrency. The next major development in consensus protocols came with Nakamoto Consensus in the [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf), by allowing consensus to be formed without needing to know the identity of each member. This solution’s major advantage is that it is robust, meaning that anyone can join or leave the network at any time, and no permission is required for a node to join the network and become a validator (hence the so-called ‘permissionless system). In this way, the Nakamoto Consensus allows for scalability, which consequently enabled the creation of Bitcoin as the first cryptocurrency.
Avalanche Consensus combines the best elements of both Classical Consensus and Nakamoto Consensus to create a new, leaderless blockchain that is scalable, robust, and green.
