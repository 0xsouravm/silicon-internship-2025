use sha2::{Sha256, Digest};

fn main() {
    let mut my_blockchain = Blockchain::new();

    my_blockchain.display();

    my_blockchain.add_blocks("Alice has 100$".to_string());
    my_blockchain.display();
    my_blockchain.add_blocks("Alice sends 50$ to Bob".to_string());
    my_blockchain.display();
    my_blockchain.add_blocks("Bob sends 20$ to Charlie".to_string());
    

    my_blockchain.display();


    my_blockchain.is_valid();

    // tampering data
    my_blockchain.blocks[2].data = "Alice sends 10$ to Bob".to_string();

    my_blockchain.is_valid();
}


/// CORE CHAIN PROPERTIES

/// 1. Immutability
/// 
/// Once data is added to block and block is successfully a part
///  of the chain then the data becomes immutable
/// 
/// Original chain
/// 
/// Block 1: Alice has 100$ (alice: 100$)
/// Block 2: Alice sends 50$ to Bob(alice: 50$, bob: 50$)
/// Block 3 : Bob sends 20$ to Charlie(bob: 30$, charlie: 20$)
/// 
/// 
/// if i am trying to change a specific block data block 2
/// Block 1: Alice has 100$ (alice: 100$)
/// Block 2: Alice sends 10$ to Bob(alice: 40$, bob: 10$)
/// Block 3 : Bob sends 20$ to Charlie(bob: 30$, charlie: 20$)
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
    pub previous_hash: String,
    pub hash: String
}

impl Block{
    pub fn new(data: String, previous_hash: String) -> Self {
        // let hash = Self::calculate_hash(&data, &previous_hash);
        // Block {
        //     data,
        //     previous_hash,
        //     hash,
        // }

        // construct a block
        let mut block = Block {
            data,
            previous_hash,
            hash: String::new(),
        };
        // update the hash
        block.hash = block.calculate_hash();
        // return the block
        block
    }

    pub fn calculate_hash(&self) -> String {
        let combined = format!("{}{}", self.data, self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(combined);
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
            let previous = &self.blocks[i - 1];

            // if my own hash is correct
            if current.hash  != current.calculate_hash() {
                println!("Invalid block at index {}: Hash mismatch", i);
                return false;
            }

            // i fprev hash is correct
            if current.previous_hash != previous.hash {
                println!("Invalid block at index {}: Previous hash mismatch", i);
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
            println!("Previous Hash: {}", block.previous_hash);
            println!("Data: {}", block.data);
            println!("===============================");
        }
    }
}

// 1 2 3 4 
// add 5 th block
// get hash of latest block 
// latest is 4
// hash is 4th block.hash
// 5th block is constructed
// add 5th block to chain