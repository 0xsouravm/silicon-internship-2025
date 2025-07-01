use crate::merkle_node::MerkleNode;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: Vec<MerkleNode>
}

impl MerkleTree {
    pub fn new(data_items: Vec<Vec<u8>>) -> Self {
        if data_items.is_empty() {
            return MerkleTree {
                root: None,
                leaves: Vec::new()
            };
        }

        let mut leaves: Vec<MerkleNode> = 
            data_items
                .into_iter()
                .map(MerkleNode::new_leaf)
                .collect();
        
        if leaves.len() == 1 {
            return MerkleTree {
                root: Some(leaves[0].clone()),
                leaves
            }
        }

        if leaves.len() % 2 == 1 {
            leaves.push(leaves.last().unwrap().clone());
        }

        let leaves_copy = leaves.clone();
        let root = Some(MerkleTree::build_tree(leaves));

        MerkleTree {
            root,
            leaves: leaves_copy
        }
    }

    fn build_tree(nodes: Vec<MerkleNode>) -> MerkleNode {
        if nodes.len() == 1 {
            return nodes[0].clone();
        }

        let mut next_level = Vec::new();

        for chunk in nodes.chunks(2) {
            if chunk.len() == 2 {
                let left = chunk[0].clone();
                let right = chunk[1].clone();
                let branch_node = MerkleNode::new_branch(left, right);
                next_level.push(branch_node);
            } else {
                next_level.push(chunk[0].clone());
            }
        }

        // println!("Next level nodes: {:?}", next_level);
        MerkleTree::build_tree(next_level)
    }

    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash())
    }

    pub fn root_hash_hex(&self) -> String {
        match self.root_hash() {
            Some(hash) => hex::encode(hash),
            None => String::from("Root hash empty")
        }
    }

    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}


// 34 56 78 89 23 90 12 39

// [34 56] -> 33
// [78 89] -> 67
// [23 90] -> 32
// [12 39] -> 100

// [33, 67, 32, 100]

// [33, 67] -> 102
// [32, 100] -> 132

// [102, 132] -> 234