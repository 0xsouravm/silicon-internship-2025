use sha2::{Sha256, Digest};
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
}

impl Transaction {
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

#[derive(Debug, Clone)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub block_number: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String, block_number: u64) -> Self {
        let mut block = Block {
            transactions,
            previous_hash,
            hash: String::new(),
            block_number,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let transactions_data = self.transactions
            .iter()
            .map(|tx| format!("{}{}{}{}",tx.from, tx.to, tx.amount, tx.fee))
            .collect::<Vec<String>>()
            .join("");
        
        let combined = format!("{}{}{}", transactions_data, self.previous_hash, self.block_number);
        let mut hasher = Sha256::new();
        hasher.update(combined);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub wallet: Wallet,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            wallet: Wallet::new(),
        };

        // Create genesis block
        let genesis_block = Block::new(Vec::new(), "0".to_string(), 0);
        blockchain.blocks.push(genesis_block);

        blockchain
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> bool {
        if transactions.is_empty() {
            println!("Cannot add empty block");
            return false;
        }

        // Validate all transactions before adding block
        for transaction in &transactions {
            if !self.wallet.validate_transaction(transaction) {
                println!("Block rejected: Invalid transaction {}", transaction.display());
                return false;
            }
        }

        let latest_block_hash = self.blocks.last().unwrap().hash.clone();
        let block_number = self.blocks.len() as u64;
        let new_block = Block::new(transactions.clone(), latest_block_hash, block_number);

        // Process all transactions
        for transaction in &transactions {
            self.wallet.process_transaction(transaction);
        }

        self.blocks.push(new_block);
        println!("Block {} added successfully with {} transactions", 
                block_number, transactions.len());
        true
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // Validate current block hash
            if current.hash != current.calculate_hash() {
                println!("Invalid block at index {}: Hash mismatch", i);
                return false;
            }

            // Validate previous hash link
            if current.previous_hash != previous.hash {
                println!("Invalid block at index {}: Previous hash mismatch", i);
                return false;
            }
        }
        println!("Blockchain is valid");
        true
    }

    pub fn display(&self) {
        println!("Blockchain Status:");
        println!("=================");
        for block in &self.blocks {
            println!("Block {}:", block.block_number);
            println!("  Hash: {}", if block.hash.len() > 16 { &block.hash[..16] } else { &block.hash });
            println!("  Previous Hash: {}", if block.previous_hash.len() > 16 { &block.previous_hash[..16] } else { &block.previous_hash });
            println!("  Transactions: {}", block.transactions.len());
            
            for (i, tx) in block.transactions.iter().enumerate() {
                println!("    {}. {}", i + 1, tx.display());
            }
            println!();
        }
        
        self.wallet.display_balances();
    }
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub balances: HashMap<String, u64>,
}

impl Wallet {
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        balances.insert("Alice".to_string(), 100);
        balances.insert("Bob".to_string(), 50);
        balances.insert("Charlie".to_string(), 20);
        balances.insert("Dave".to_string(), 10);
        
        Wallet { balances }
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        if let Some(balance) = self.balances.get(&transaction.from) {
            let total_cost = transaction.amount + transaction.fee;
            if *balance >= total_cost {
                return true;
            } else {
                println!("Insufficient balance for {}: has {}, needs {}", 
                    transaction.from, balance, total_cost);
            }
        } else {
            println!("Sender {} does not exist", transaction.from);
        }
        false
    }

    pub fn process_transaction(&mut self, transaction: &Transaction) {
        // Deduct from sender
        if let Some(balance) = self.balances.get_mut(&transaction.from) {
            *balance -= transaction.amount + transaction.fee;
        }

        // Add to recipient
        *self.balances.entry(transaction.to.clone()).or_insert(0) += transaction.amount;
        
        println!("Processed: {}", transaction.display());
    }

    pub fn display_balances(&self) {
        println!("Wallet Balances:");
        println!("---------------");
        for (name, balance) in &self.balances {
            println!("  {}: {} coins", name, balance);
        }
        println!();
    }

    #[allow(dead_code)]
    pub fn get_balance(&self, name: &str) -> u64 {
        *self.balances.get(name).unwrap_or(&0)
    }
}

pub struct TransactionPool {
    pub transactions: Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self {
        TransactionPool {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction, wallet: &Wallet) -> bool {
        if wallet.validate_transaction(&transaction) {
            self.transactions.push(transaction);
            println!("Transaction added to pool");
            true
        } else {
            println!("Transaction failed validation");
            false
        }
    }

    pub fn get_best_transactions(&mut self, count: usize) -> Vec<Transaction> {
        // Sort by fee (highest first)
        self.transactions.sort_by(|a, b| b.fee.cmp(&a.fee));
        
        let selected: Vec<Transaction> = self.transactions.iter().take(count).cloned().collect();
        self.transactions = self.transactions.iter().skip(count).cloned().collect();
        
        selected
    }

    pub fn display_status(&self) {
        println!("Transaction Pool ({} pending transactions):", self.transactions.len());
        for (i, transaction) in self.transactions.iter().enumerate() {
            println!("  {}. {}", i + 1, transaction.display());
        }
        println!();
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.transactions.len()
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub is_malicious: bool,
}

impl Node {
    pub fn new_honest(name: &str) -> Self {
        Node { 
            name: name.to_string(), 
            is_malicious: false 
        }
    }
    
    pub fn new_malicious(name: &str) -> Self {
        Node { 
            name: name.to_string(), 
            is_malicious: true 
        }
    }
    
    pub fn vote(&self, transactions: &[Transaction]) -> bool {
        let approve = !self.is_malicious;
        println!("Node {} votes {} on block with {} transactions", 
            self.name, 
            if approve { "YES" } else { "NO" }, 
            transactions.len()
        );
        approve
    }
}