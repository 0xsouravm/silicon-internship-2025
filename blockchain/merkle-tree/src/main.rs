mod merkle_node;
mod merkle_tree;

pub use merkle_node::MerkleNode;
pub use merkle_tree::MerkleTree;

fn main() {
    let transactions = vec![
        b"Alice -> Bob = 15 DOT".to_vec(),
        b"Bob -> Charlie = 2 DOT".to_vec(),
        b"Dave -> Emily = 50 DOT".to_vec(),
        b"Gamma -> Echo = 3 DOT".to_vec(),
    ];

    println!("Transactions: {:?}", transactions);

    let merkle_tree = MerkleTree::new(transactions.clone());
    println!("Merkle Tree Root Hash: {:?}", merkle_tree.root_hash_hex());
}
