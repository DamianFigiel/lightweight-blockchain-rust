use crate::block::{Block, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: vec![Blockchain::create_genesis_block()] }
    }

    fn create_genesis_block() -> Block {
        Block::new(0, Blockchain::current_timestamp(), "0".into(), vec![])
    }

    fn current_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    pub fn add_block(&mut self, trasactions: Vec<Transaction>) {
        let latest_block = self.chain.last().unwrap();
        let new_block = Block::new(
            latest_block.index + 1,
            Blockchain::current_timestamp(),
            latest_block.hash.clone(),
            trasactions,
        )
        self.chain.push(new_block);
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.previous_hash != previous.hash || current.calculate_hash() != current.hash {
                return false;
            }     
        }
        true
    }

}