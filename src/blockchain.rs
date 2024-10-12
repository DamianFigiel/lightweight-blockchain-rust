use crate::block::{Block, Transaction};
use rand::Rng;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Self {
            blocks: vec![],
            difficulty: 4,
        };
        let genesis_block = Block::new(0, String::from("0"), vec![], 0);
        blockchain.blocks.push(genesis_block);
        blockchain
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.blocks.last().unwrap();
        let mut rng = rand::thread_rng();
        let mut nonce = rng.gen::<u64>();

        loop {
            let block = Block::new(
                self.blocks.len() as u64,
                previous_block.hash(),
                transactions.clone(),
                nonce,
            );

            if &block.hash()[..self.difficulty] == "0000" {
                self.blocks.push(block);
                break;
            }

            nonce += 1;
        }
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        &block.hash()[..self.difficulty] == "0000"
    }

    pub fn validate_transaction(&self, tx: &Transaction) -> bool {
        tx.amount > 0
    }
}
