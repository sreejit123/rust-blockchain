use std::time::{SystemTime, UNIX_EPOCH};

use crate::block::{Block, BlockTrait};

pub trait BlockchainTrait {
    fn mine(&mut self, b : &mut Block);
}

#[derive(Debug)]
pub struct Blockchain {
    chain : Vec<Block>,
}

impl Blockchain {
    pub fn new () -> Blockchain {
        let mut chain : Vec<Block> = Vec::new();

        let mut genesis_block = Block::default();
        genesis_block.prev_hash = String::from("0");
        genesis_block.nonce = 0;
        genesis_block.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        genesis_block.hash = Block::calculate_hash(&genesis_block);



        chain.push(genesis_block);

        Blockchain {
            chain
        }
    }
}

impl BlockchainTrait for Blockchain {

    fn mine(&mut self, b : &mut Block) {
        let prev_block = &self.chain[self.chain.len() - 1];
        b.prev_hash = prev_block.hash.to_string();
        b.nonce = 0;
        let mut hash = Block::calculate_hash(b);

        while !hash.starts_with("0000") {
            b.nonce += 1;
            hash = Block::calculate_hash(b);
        }

        let block = Block {
            hash: hash,
            prev_hash: b.prev_hash.to_string(),
            nonce: b.nonce,
            timestamp: b.timestamp,
        };

        self.chain.push(block);

    }
}