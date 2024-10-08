use serde::{Serialize, Deserialize};
use ring::digest::{digest, SHA256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    index: u64,
    timestamp: u64,
    previous_hash: String,
    trasactions: Vec<Transaction>,
    hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u64, previous_hash: String, trasactions: Vec<Transaction>) -> Self {
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            trasactions,
            hash: String::new(),
        }
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let block_data = serde_json::to_string(&self).unwrap();
        let hash = digest(&SHA256, block_data.as_bytes());
        hex::encode(hash)
    }
}