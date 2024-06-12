/*
The consensus mechanism is probabilistic and relies on repeated sub-sampled voting.
Nodes repeatedly query a random subset of other nodes in the network and move towards consensus based on the majority responses they receive.
This Rust implementation provides a basic but functional simulation of the Avalanche consensus mechanism, focusing on the core ideas of state management, node interaction, and iterative consensus building through network communication.
*/

extern crate byteorder;
extern crate hex;
extern crate rand;
extern crate ring;

use byteorder::{LittleEndian, WriteBytesExt};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

use ring::digest;

use std::collections::HashMap;
use std::sync::{mpsc::{channel, Receiver, Sender},
                Arc,
                Mutex};
use std::thread;
use std::time::Duration;

/// Hardcoded tuning parameters for the algorithm.
pub const SAMPLES: usize = 4;
pub const MAX_EPOCHS: u32 = 4;
pub const THRESHOLD: f32 = 0.75;
pub const CONVICTION_THRESHOLD: f32 = 0.75;

fn main() {
    let mut net = Network::new(10);
    net.run();

    loop {
        // Transactions are generated and sent to random nodes in the network
        let tx = Transaction::random();
        println!("sending new transaction into the network {}", &tx.hash());

        // Pick a random node in the network and let the node handle the random transaction.
        let id = thread_rng().gen_range(0..net.nodes.len()) as u64;
        // Transactions are verified by nodes independently. If the transaction data is less than 7, it's considered valid; otherwise, it's invalid.
        let node = net.nodes.get_mut(&id).unwrap();
        node.lock()
            .unwrap()
            .handle_message(0, &Message::Transaction(tx));

        thread::sleep(Duration::from_millis(500)); // cpu ded
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Hash(Vec<u8>);

impl Hash {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl ::std::fmt::Display for Hash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl ::std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

#[derive(Debug)]
enum Message {
    Query(QueryMessage),
    QueryResponse((u64, QueryResponse)),
    Transaction(Transaction),
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Valid,
    Invalid,
}

/*
Nodes handle incoming messages (Query, QueryResponse, Transaction) and 
process them according to the Avalanche protocol rules.
*/

#[derive(Debug)]
struct QueryResponse {
    hash: Hash,
    status: Status,
}

#[derive(Debug)]
struct QueryMessage {
    tx: Transaction,
    status: Status,
}

#[derive(Debug, Clone)]
struct Transaction {
    nonce: u64,
    data: i32,
}

impl Transaction {
    fn random() -> Self {
        let mut rng = thread_rng();
        Transaction {
            nonce: rand::random::<u64>(),
            data: rng.gen_range(0..10),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buf = vec![];
        buf.write_u64::<LittleEndian>(self.nonce).unwrap();
        buf
    }

    fn hash(&self) -> Hash {
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(&self.serialize());
        Hash(ctx.finish().as_ref().to_vec())
    }
}

// manages nodes and handles message passing between them using channels
#[derive(Debug)]
struct Network {
    nodes: HashMap<u64, Arc<Mutex<Node>>>,
    receiver: Arc<Mutex<Receiver<(u64, Message)>>>,
}

impl Network {
    /// Create a new network with `n` participating nodes.
    fn new(n: u64) -> Self {
        let (sender, receiver) = channel();
        Network {
            nodes: (0..n)
                .map(|id| (id, Arc::new(Mutex::new(Node::new(id, sender.clone())))))
                .collect(),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    fn run(&self) {
        let receiver = self.receiver.clone();
        let mut nodes = self.nodes.clone();

        thread::spawn(move || loop {
            let (origin, msg) = receiver.lock().unwrap().recv().unwrap();
            match msg {
                Message::Query(ref _msg) => {
                    let mut sampled = sample_nodes(&nodes, SAMPLES, origin);
                    sampled
                        .iter()
                        .map(|id| {
                            nodes
                                .get_mut(&id)
                                .unwrap()
                                .lock()
                                .unwrap()
                                .handle_message(origin, &msg)
                        })
                        .collect::<Vec<_>>();
                }
                Message::QueryResponse((to, ref _msg)) => {
                    let mut node = nodes.get_mut(&to).unwrap();
                    node.lock().unwrap().handle_message(origin, &msg);
                }
                _ => unreachable!(),
            }
        });
    }
}

fn sample_nodes(nodes: &HashMap<u64, Arc<Mutex<Node>>>, n: usize, excl: u64) -> Vec<u64> {
    let ids: Vec<u64> = nodes
        .iter()
        .filter(|(&id, _)| id != excl)
        .map(|(id, _)| *id)
        .collect();
    let mut rng = thread_rng();
    let sampled_ids = ids.as_slice().choose_multiple(&mut rng, n).cloned().collect();
    sampled_ids
}

// Each Node has a mempool to manage transaction states 
#[derive(Debug, Clone)]
struct TxState {
    epoch: u32,
    tx: Transaction,
    status: Status,
    responses: Vec<Status>,
    is_final: bool,

    /// 1. Each node maintains a counter cnt
    /// 2. Upon every color change, the node resets cnt to 0
    /// 3. Upon every successful query that yields ≥ αk responses for the same
    /// color as the node, the node increments cnt.
    cnt_valid: u32,
    cnt_invalid: u32,
    cnt: u32,

    /// Last decided status.
    last_status: Status,
}

impl TxState {
    fn new(tx: Transaction, status: Status) -> Self {
        TxState {
            responses: Vec::new(),
            is_final: false,
            last_status: Status::Invalid,
            epoch: 0,
            cnt_valid: 0,
            cnt_invalid: 0,
            cnt: 0,
            tx,
            status,
        }
    }

    fn incr_status(&mut self, s: &Status) -> u32 {
        match s {
            Status::Valid => {
                self.cnt_valid += 1;
                self.cnt_valid
            }
            Status::Invalid => {
                self.cnt_invalid += 1;
                self.cnt_invalid
            }
        }
    }

    fn status_count(&self, s: &Status) -> u32 {
        match s {
            Status::Valid => self.cnt_valid,
            Status::Invalid => self.cnt_invalid,
        }
    }

    fn advance(&mut self) {
        self.epoch += 1;
        self.responses.clear();
    }
}

#[derive(Debug, Clone)]
struct Node {
    mempool: HashMap<Hash, TxState>,
    id: u64,
    sender: Sender<(u64, Message)>,
}

impl Node {
    fn new(id: u64, sender: Sender<(u64, Message)>) -> Self {
        Node {
            id,
            sender,
            mempool: HashMap::new(),
        }
    }

    fn handle_message(&mut self, origin: u64, msg: &Message) {
        println!("node {} recv from {} => {:?}", self.id, origin, msg);

        match msg {
            Message::Query(ref msg) => self.handle_query(origin, msg),
            Message::QueryResponse((_to, ref msg)) => {
                if let Some((hash, status)) = self.handle_query_response(msg) {
                    println!("node {} got decision {:?} for tx {}", self.id, status, hash);
                };
            }
            Message::Transaction(tx) => self.handle_transaction(tx),
        }
    }

    /// Upon receiving a query, an uncolored node adopts the color in the query,
    /// responds with that color, and initiates its own query, whereas a colored
    /// node simply responds with its current color.
    fn handle_query(&mut self, origin: u64, msg: &QueryMessage) {
        // TODO: This can be so much cleaner, just fighting to much with compiler!!
        let state = if !self.mempool.contains_key(&msg.tx.hash()) {
            let state = TxState::new(msg.tx.clone(), msg.status.clone());
            // Nodes receive a query about a transaction. If the transaction is new, it's added to the mempool and queried further. Responses are sent back to the querying node.
            self.mempool.insert(msg.tx.hash(), state.clone());
            self.send_query(msg.tx.clone(), msg.status.clone());
            state
        } else {
            let state = self.mempool.get(&msg.tx.hash()).unwrap();
            state.clone()
        };
        self.send_response(origin, state.tx.hash(), state.status.clone());
    }

    /// If k responses are not received within a time bound, the node picks an
    /// additional sample from the remaining nodes uniformly at random and queries
    /// them until it collects all responses.     
    /// TODO: timeout + error handling + factor some pieces out of this method!
    fn handle_query_response(&mut self, msg: &QueryResponse) -> Option<(Hash, Status)> {
        // Nodes process responses to their queries and update their internal state.
        {
            let state = self.mempool.get_mut(&msg.hash).unwrap();
            // If the state is considered final we dont handle this response anymore.
            if state.is_final {
                return None;
            }
            state.responses.push(msg.status.clone());

            let n = state
                .responses
                .iter()
                .filter(|&status| status == &msg.status)
                .count();

            // If responses meet the threshold criteria, the node updates its internal state and may decide on the transaction's status.
            if n >= (THRESHOLD * SAMPLES as f32) as usize {
                // Increment the confidence of the received status.
                let cnt = state.incr_status(&msg.status);
                // Get the confidence of our current status.
                let our_status_cnt = state.status_count(&state.status);

                // If the confidence of the received status is higher then ours we
                // flip to that status.
                if cnt > our_status_cnt {
                    state.status = msg.status.clone();
                    state.last_status = state.status.clone();
                }

                if msg.status != state.last_status {
                    state.last_status = msg.status.clone();
                    state.cnt = 0;
                } else {
                    state.cnt += 1;
                    // We only accept the color (move to the next epoch) if the
                    // counter is higher the the conviction threshold.
                    if state.cnt > (CONVICTION_THRESHOLD * SAMPLES as f32) as u32 {
                        state.advance();
                        if state.epoch == MAX_EPOCHS {
                            state.is_final = true;
                            return Some((state.tx.hash(), state.status.clone()));
                        }
                    }
                }
            }
        }

        let state = self.mempool.get(&msg.hash).unwrap();
        self.send_query(state.tx.clone(), state.status.clone());
        None
    }

    // When a new transaction is received, it's verified and added to the mempool, and the querying process begins.
    fn handle_transaction(&mut self, tx: &Transaction) {
        // Verify transaction ourself.
        let status = self.verify_transaction(tx);

        // Add the tx to our mempool.
        self.mempool
            .insert(tx.hash(), TxState::new(tx.clone(), status.clone()));
        self.send_query(tx.clone(), status.clone());
    }

    // sending a query to another node
    fn send_query(&self, tx: Transaction, status: Status) {
        let msg = Message::Query(QueryMessage {
            tx: tx,
            status: status,
        });
        self.sender.send((self.id, msg));
    }

    // sending a response to another node
    fn send_response(&self, to: u64, hash: Hash, status: Status) {
        let msg = Message::QueryResponse((to, QueryResponse { hash, status }));
        self.sender.send((self.id, msg));
    }

    fn verify_transaction(&self, tx: &Transaction) -> Status {
        match tx.data < 7 {
            true => Status::Valid,
            false => Status::Invalid,
        }
    }
}
