mod blockchain;

use blockchain::{Blockchain, Transaction, TransactionPool, Node};

fn run_consensus(nodes: &[Node], blockchain: &mut Blockchain, transaction_pool: &mut TransactionPool) -> bool {
    if transaction_pool.is_empty() {
        println!("No transactions to process");
        return false;
    }

    // Get transactions from pool (max 3 per block for this example)
    let transactions = transaction_pool.get_best_transactions(3);
    
    if transactions.is_empty() {
        return false;
    }

    println!("Consensus voting on block with {} transactions:", transactions.len());
    for tx in &transactions {
        println!("  - {}", tx.display());
    }
    
    // Voting process
    let mut yes_votes = 0;
    for node in nodes {
        if node.vote(&transactions) {
            yes_votes += 1;
        }
    }
    
    let total_votes = nodes.len();
    let majority_needed = (total_votes / 2) + 1;
    
    println!("Results: {} YES out of {} votes (need {})", yes_votes, total_votes, majority_needed);
    
    // Check if majority approved
    if yes_votes >= majority_needed {
        println!("ACCEPTED! Adding block to blockchain");
        blockchain.add_block(transactions);
        true
    } else {
        println!("REJECTED! Block not added");
        // Return transactions to pool
        for tx in transactions {
            transaction_pool.transactions.push(tx);
        }
        false
    }
}

fn demonstrate_simple_blockchain() {
    println!("PART 1: Simple Blockchain Without Consensus");
    println!("===========================================");
    
    let mut simple_blockchain = Blockchain::new();
    
    // Create some transactions
    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 30, 1);
    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 15, 1);
    let tx3 = Transaction::new("Alice".to_string(), "Dave".to_string(), 999, 1); // This should fail
    
    println!("Adding transactions directly (no validation):");
    
    // Add blocks directly without validation (old way)
    simple_blockchain.add_block(vec![tx1]);
    simple_blockchain.add_block(vec![tx2]);
    simple_blockchain.add_block(vec![tx3]); // This will fail validation
    
    simple_blockchain.display();
    
    println!("Problem: Invalid transactions can be rejected, but there's no consensus mechanism");
    println!();
}

fn demonstrate_consensus_blockchain() {
    println!("PART 2: Consensus Blockchain with Transaction Pool");
    println!("================================================");
    
    let mut consensus_blockchain = Blockchain::new();
    let mut transaction_pool = TransactionPool::new();
    
    // Create nodes for consensus
    let nodes = vec![
        Node::new_honest("Alice"),
        Node::new_honest("Bob"), 
        Node::new_malicious("Mallory"),
        Node::new_honest("Charlie"),
    ];
    
    println!("Network nodes: Alice (honest), Bob (honest), Mallory (malicious), Charlie (honest)");
    println!("Majority needed: 3 out of 4 votes");
    println!();
    
    consensus_blockchain.wallet.display_balances();
    
    // Create and add transactions to pool
    println!("Adding transactions to pool:");
    
    let transactions = vec![
        Transaction::new("Alice".to_string(), "Bob".to_string(), 25, 2),
        Transaction::new("Bob".to_string(), "Charlie".to_string(), 10, 1),
        Transaction::new("Charlie".to_string(), "Dave".to_string(), 5, 1),
        Transaction::new("Alice".to_string(), "Dave".to_string(), 15, 3), // Higher fee
        Transaction::new("Bob".to_string(), "Alice".to_string(), 8, 1),
        Transaction::new("Dave".to_string(), "Bob".to_string(), 3, 1),
    ];
    
    for tx in transactions {
        transaction_pool.add_transaction(tx, &consensus_blockchain.wallet);
    }
    
    transaction_pool.display_status();
    
    // Process blocks with consensus
    println!("Processing blocks with consensus:");
    println!("=================================");
    
    let mut block_count = 1;
    while !transaction_pool.is_empty() && block_count <= 3 {
        println!("Block {} consensus:", block_count);
        println!("-----------------");
        
        let success = run_consensus(&nodes, &mut consensus_blockchain, &mut transaction_pool);
        
        if success {
            block_count += 1;
        }
        
        println!();
        
        if !transaction_pool.is_empty() {
            transaction_pool.display_status();
        }
    }
    
    // Final blockchain state
    consensus_blockchain.display();
    
    // Validate blockchain integrity
    println!("Blockchain Validation:");
    println!("====================");
    consensus_blockchain.is_valid();
}

fn demonstrate_malicious_scenario() {
    println!("PART 3: Malicious Scenario Demonstration");
    println!("========================================");
    
    let mut blockchain = Blockchain::new();
    let mut transaction_pool = TransactionPool::new();
    
    // Create a network with majority malicious nodes
    let malicious_nodes = vec![
        Node::new_malicious("Evil1"),
        Node::new_malicious("Evil2"),
        Node::new_malicious("Evil3"),
        Node::new_honest("Alice"),
    ];
    
    println!("Network with majority malicious nodes: Evil1, Evil2, Evil3 (malicious), Alice (honest)");
    println!("This demonstrates how malicious majority can reject valid transactions");
    println!();
    
    // Add a valid transaction
    let valid_tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10, 1);
    transaction_pool.add_transaction(valid_tx, &blockchain.wallet);
    
    transaction_pool.display_status();
    
    println!("Attempting consensus with malicious majority:");
    run_consensus(&malicious_nodes, &mut blockchain, &mut transaction_pool);
    
    println!("Result: Even valid transactions can be rejected by malicious majority");
    println!("Solution: Ensure honest majority in the network");
}

fn main() {
    println!("Blockchain with Transaction Pool and Consensus Demo");
    println!("==================================================");
    println!();
    
    demonstrate_simple_blockchain();
    demonstrate_consensus_blockchain();
    demonstrate_malicious_scenario();
    
    println!();
    println!("KEY CONCEPTS DEMONSTRATED:");
    println!("=========================");
    println!("1. Transaction validation before adding to pool");
    println!("2. Fee-based transaction prioritization");
    println!("3. Consensus mechanism for block approval");
    println!("4. Wallet balance tracking and updates");
    println!("5. Protection against malicious transactions");
    println!("6. Blockchain integrity validation");
}