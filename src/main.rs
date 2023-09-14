mod block;
mod blockchain;

use std::time::{SystemTime, UNIX_EPOCH};
use crate::block::Block;
use crate::blockchain::{Blockchain, BlockchainTrait};

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();

    let mut firstBlock = Block::default();
    firstBlock.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    blockchain.mine(&mut firstBlock);

    println!("State of chain is {:?}", &blockchain);
}
