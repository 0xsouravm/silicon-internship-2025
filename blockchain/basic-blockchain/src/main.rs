use sha2::{Sha256, Digest};
use std::collections::HashMap;

// fn main() {
//     let mut my_blockchain = Blockchain::new();

//     my_blockchain.display();

//     my_blockchain.add_blocks("Arindam has 100$".to_string());
//     my_blockchain.display();
//     my_blockchain.add_blocks("Arindam sends 50$ to Yuvi".to_string());
//     my_blockchain.display();
//     my_blockchain.add_blocks("Yuvi sends 20$ to Abhilasha".to_string());
    

//     my_blockchain.display();


//     my_blockchain.is_valid();

//     // tampering data
//     my_blockchain.blocks[2].data = "Arindam sends 10$ to Yuvi".to_string();

//     my_blockchain.is_valid();
// }


/// CORE CHAIN PROPERTIES

/// 1. Immutability
/// 
/// Once data is added to block and block is successfully a part
///  of the chain then the data becomes immutable
/// 
/// Original chain
/// 
/// Block 1: Arindam has 100$ (Arindam: 100$)
/// Block 2: Arindam sends 50$ to Yuvi(Arindam: 50$, Yuvi: 50$)
/// Block 3 : Yuvi sends 20$ to Abhilasha(Yuvi: 30$, Abhilasha: 20$)
/// 
/// 
/// if i am trying to change a specific block data block 2
/// Block 1: Arindam has 100$ (Arindam: 100$)
/// Block 2: Arindam sends 10$ to Yuvi(Arindam: 40$, Yuvi: 10$)
/// Block 3 : Yuvi sends 20$ to Abhilasha(Yuvi: 30$, Abhilasha: 20$)
/// 
/// 
/// Domino effect will be:
/// Change block 2 -> Hash changed -> Block 3 reference will be invalid 
/// -> Block 3 wil be invalid -> Block 4 will be invalid
///  
/// 
/// 1 -> 2 -> 3 -> 4 -> 5 node 2
///  1 -> 2 -> 3 -> 4 -> 5 node 3
/// 1 -> 2 -> 3 -> 4 -> 5 node 4
/// 1 -> 2 node 1 (malicious node)
/// 
/// 
/// 2. Cryptographic Hash linking
/// 
/// 3. Distributed consent mechanism
/// 


#[derive(Debug, Clone)]
// Describe a block structure to store block data and create blocks
pub struct Block{
    pub data: String,
    pub prev_hash: String,
    pub hash: String
}

impl Block{
    pub fn new(data: String, prev_hash: String) -> Self {
        // let hash = Self::calc_hash(&data, &prev_hash);
        // Block {
        //     data,
        //     prev_hash,
        //     hash,
        // }

        // construct a block
        let mut block = Block {
            data,
            prev_hash,
            hash: String::new(),
        };
        // update the hash
        block.hash = block.calc_hash();
        // return the block
        block
    }

    pub fn calc_hash(&self) -> String {
        let combo = format!("{}{}", self.data, self.prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(combo);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}


#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
// 1 2 3 4 5
impl Blockchain {
    pub fn new() -> Self{

        let mut blockchain = Blockchain {
            blocks: Vec::new()
        };

        // add a genesis block
        let genesis_block = Block::new("First block on the chain".to_string(), "0".to_string());
        blockchain.blocks.push(genesis_block);

        blockchain
    }

    pub fn add_blocks(&mut self, data: String) {
        // old chain state 4 
        let latest_block_hash = self.blocks.last().unwrap().hash.clone();

        let new_block = Block::new(data, latest_block_hash);

        self.blocks.push(new_block);
        // 5 block

        println!("Added new block ");
    }

 // 0 1 2 3 4 5 6 7
    pub fn is_valid (&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let prev = &self.blocks[i - 1];

            // if my own hash is correct
            if current.hash  != current.calc_hash() {
                println!("Invalid block at index {}: Hash mismatch", i);
                return false;
            }

            // if prev hash is correct
            if current.prev_hash != prev.hash {
                println!("Invalid block at index {}: prev hash mismatch", i);
                return false;
            }
        }
        println!("Blockchain is valid");
        true
    }

    pub fn display(&self) {
        println!("Blockchain:");
        for (i, block) in self.blocks.iter().enumerate() {
            println!("Block {}:", i + 1);
            println!("---------------");
            println!("Hash: {}", block.hash);
            println!("previous Hash: {}", block.prev_hash);
            println!("Data: {}", block.data);
            println!("===============================");
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub is_malicious: bool,
}

impl Node {
    pub fn new_honest(name: &str) -> Self {
        Node { name: name.to_string(), is_malicious: false }
    }
    
    pub fn new_malicious(name: &str) -> Self {
        Node { name: name.to_string(), is_malicious: true }
    }
    
    pub fn vote(&self, proposal: &str) -> bool {
        let approve = if self.is_malicious { false } else { true };
        println!("🗳️  {} votes {} on '{}'", 
            self.name, 
            if approve { "YES" } else { "NO" }, 
            proposal
        );
        approve
    }
}

fn run_consensus(nodes: &[Node], blockchain: &mut Blockchain, proposal: String) {
    println!("📝 Proposal: '{}'", proposal);
    
    // Everyone votes
    let mut yes_votes = 0;
    for node in nodes {
        if node.vote(&proposal) {
            yes_votes += 1;
        }
    }
    
    let total_votes = nodes.len();
    let majority_needed = (total_votes / 2) + 1;
    
    println!("📊 Results: {} YES out of {} votes", yes_votes, total_votes);
    
    // Check if majority
    if yes_votes >= majority_needed {
        println!("✅ ACCEPTED! Adding to blockchain");
        blockchain.add_blocks(proposal);
    } else {
        println!("❌ REJECTED! Not adding to blockchain");
    }
    println!();
}

fn main() {
    println!("🚀 Simple Blockchain vs Consensus Blockchain");
    println!("============================================\n");
    
    
    println!("📖 PART 1: Simple Blockchain (like yesterday)");
    println!("----------------------------------------------");
    
    let mut simple_blockchain = Blockchain::new();
    
    // Just add blocks directly (no voting)
    simple_blockchain.add_blocks("Arindam sends $50 to Yuvi".to_string());
    simple_blockchain.add_blocks("Yuvi sends $20 to Abhilasha".to_string());
    simple_blockchain.add_blocks("Malicious: Free money for everyone!".to_string());
    
    simple_blockchain.display();
    
    println!("⚠️  Problem: Anyone can add anything! No security!\n");
    
    
    println!("📖 PART 2: Consensus Blockchain (today's improvement)");
    println!("-----------------------------------------------------");
    
    // Create nodes
    let nodes = vec![
        Node::new_honest("Arindam"),
        Node::new_honest("Yuvi"), 
        Node::new_malicious("Mallory"),
    ];
    
    println!("👥 Nodes: Arindam (honest), Yuvi (honest), Mallory (malicious)");
    println!("📐 Need majority: 2 out of 3 votes\n");
    
    let mut consensus_blockchain = Blockchain::new();
    
    // Now we need consensus to add blocks
    run_consensus(&nodes, &mut consensus_blockchain, "Arindam sends $50 to Yuvi".to_string());
    run_consensus(&nodes, &mut consensus_blockchain, "Yuvi sends $20 to Abhilasha".to_string());
    run_consensus(&nodes, &mut consensus_blockchain, "Malicious: Free money for everyone!".to_string());
    run_consensus(&nodes, &mut consensus_blockchain, "Abhilasha sends $10 to Dave".to_string());
    
    consensus_blockchain.display();
    
    
    println!("🔍 COMPARISON:");
    println!("Simple Blockchain:    {} blocks (including malicious)", simple_blockchain.blocks.len());
    println!("Consensus Blockchain: {} blocks (malicious blocked!)", consensus_blockchain.blocks.len());
    
    println!("\n🎓 LESSON:");
    println!("✅ Consensus prevents malicious blocks");
    println!("✅ Majority rule defeats minority attackers");
    println!("✅ This is how real blockchains work!");
    
    println!("\n\n📖 PART 3: Transaction Processing Demo");
    println!("======================================");
    
    // Create wallet and transaction pool
    let mut wallet = Wallet::new();
    let mut tx_pool = TxnPool::new();
    
    println!("Initial wallet state:");
    wallet.display_balances();
    println!();
    
  
    let tx1 = Transaction::new("Arindam".to_string(), "Yuvi".to_string(), 30, 2);
    let tx2 = Transaction::new("Yuvi".to_string(), "Abhilasha".to_string(), 15, 1);
    let tx3 = Transaction::new("Abhilasha".to_string(), "Dave".to_string(), 100, 5); // Should fail
    

    tx_pool.add_transaction(tx1, &wallet);
    tx_pool.add_transaction(tx2, &wallet);
    tx_pool.add_transaction(tx3, &wallet);
    
    println!();
    tx_pool.display_status();
    

    let best_txns = tx_pool.get_best_transaction(2);
    println!("\nProcessing {} best transactions:", best_txns.len());
    
    for tx in &best_txns {
        println!("{}", tx.display());
        wallet.process_transaction(tx);
    }
    
    println!("\nfinalize wallet state:");
    wallet.display_balances();
}


/// txn processing
// Inside the blocks that goes on to the chain 
//there are txns and txn processing is how the txn are created , 
//validated, and processed


// What is a transaction??

// In real world it is change of balance or assets 
// in blockchain anything that happens goes as a txn to the chain

/// Bank check

// pay to: Person
// amount: $100
// from: You
// signature: Your signature
// date: 2023-10-01

/// Blockchain txn

// To: Person(public key)
// Amount : $100
// From: you (public key)
// fee: $1
// signature: Your signature

// Gas fee 
// just like tax on blockchain you have to pay txn fee for every txn you do
// and this fee varies from platform to platform


/// Transaction Structure
/// 
// pub struct Transaction {
//     from: String, // sender's address
//     to: String, // recipient's address
//     amount: f64,
//     fee: f64,
//     signature: String,
// }

// 0xyth67hg59vbsjjs
// what is signing??
//  uueh488209jk33vz

// Transaction Validation

// 1. Signature check
// Take txn data
// take Arindam pub key
// validate the signature using Arindam pub key 
// if match continue else reject


// example
//  Arindam sends 50$ to Yuvi
// signature verification
// Arindam pub key is is wrong in the txn
// Arindam pubkey is correct but signature validation pass

// 1. signature verification
// 2. balance check
// 3. Double spending check
// 4. format check

// new txn -> check signature -> check balance -> double spend check -> format check -> accept to txn pool


// txn pool is the waiting area of txns
// user sends one txn
// txn is validated by the node 
// txn is added to the txn pool
// whenever the next block is created it will pick the max number of txn it can


// every 6 sec we create one block
// at the beginning of 6 sec there is one block created 
// it will wait 2/3rd of time for txn pool
// 1/3of time it will take to validate and finalize the block


// Transaction Fee

// Gas fee  = (fee rate * txn size)

// Txn lifecycle

// create the txn
// broadcast the txn
// validate the sender pub key
// validate the signature
// validate the balance
// validate double spending
// format check
// add to txn pool
// add the txn to block
// confirm the block

#[derive(Debug, Clone)]
pub struct Transaction{
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
}

impl Transaction{
    pub fn new(from: String, to: String, amount: u64, fee: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
            fee,
        }
    }

    pub fn display(&self) -> String {
        format!("Transaction: {} -> {} | Amount: {} | Fee: {}", 
            self.from, 
            self.to, 
            self.amount, 
            self.fee
        )
    }
}

pub struct TxnPool {
    pub transactions: Vec<Transaction>,
}

impl TxnPool{
    pub fn new() -> Self {
        TxnPool {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction, balances: &Wallet) -> bool {
        if balances.has_balance(&transaction.from, transaction.amount, transaction.fee) {
            self.transactions.push(transaction);
            println!("Transaction added to pool.");
            true
        } else {
            println!("Transaction failed: Insufficient balance or sender does not exist");
            false
        }
        
    }

    pub fn get_best_transaction(&mut self, count: usize) -> Vec<Transaction> {
        self.transactions.sort_by(|a, b| b.fee.cmp(&a.fee));
        let selected_txns: Vec<Transaction> = self.transactions.iter().take(count).cloned().collect();
        self.transactions = self.transactions.iter().skip(count).cloned().collect();
        selected_txns
    }

    pub fn display_status(&self) {
        println!("Transaction pool({} pending txns):", self.transactions.len());
        for txn in &self.transactions {
            println!("{}", txn.display());
        }
    }
}

pub struct Wallet{
    pub balances: HashMap<String, u64>, 
}

// pub struct Account {
//     name: String,
//     balance: u64,
// }

impl Wallet { 
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        balances.insert("Arindam".to_string(), 100);
        balances.insert("Yuvi".to_string(), 50);
        balances.insert("Abhilasha".to_string(), 20);
        Wallet { balances }
    }

    pub fn has_balance(&self, name: &str, amount: u64, fee: u64) -> bool {
        if let Some(balance) = self.balances.get(name) {
            return *balance >= amount + fee;
        }
        false
    }

    pub fn process_transaction(&mut self, tx: &Transaction) {
        if let Some(balance) = self.balances.get_mut(&tx.from) {
            *balance -= tx.amount + tx.fee;
        } 

        *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;

        // it checks for the key in the hashmap
        // if key exists go to update the value
        // if key does not exist in that case it will add the key to the hashmap with 
        // the initial value provided inside or_insert(0) and then go for update
    }

    pub fn display_balances(&self) {
        println!("Wallet Balances:");
        for (name, balance) in &self.balances {
            println!("{}: {}$", name, balance);
        }
    }
}