use sha2::{ Digest, Sha256 };
use std::fmt;

#[derive(Clone, Debug)]
pub enum MerkleNode {
    Leaf {
        data: Vec<u8>,
        hash: Vec<u8>
    },
    Branch {
        left: Box<MerkleNode>,
        right: Box<MerkleNode>,
        hash: Vec<u8>
    }
}

impl MerkleNode {
    pub fn new_leaf(data: Vec<u8>) -> Self {
        let hash = Sha256::digest(&data).to_vec();
        MerkleNode::Leaf {
            data,
            hash
        }
    }

    pub fn new_branch(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Sha256::new();

        let left_hash = &left.hash();
        let right_hash = &right.hash();

        hasher.update(left_hash);
        hasher.update(right_hash);
        let hash = hasher.finalize().to_vec();

        MerkleNode::Branch {
            left: Box::new(left),
            right: Box::new(right),
            hash
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        match self {
            MerkleNode::Leaf { hash, .. } => hash.clone(),
            MerkleNode::Branch { hash, .. } => hash.clone()
        }
    }
}