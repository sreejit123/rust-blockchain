use sha2::{Sha256, Digest};

pub trait BlockTrait {
    fn calculate_hash(b : &Block) -> String;

}

#[derive(PartialEq, Debug, Default)]
pub struct Block {
    pub nonce : u128,
    pub hash : String,
    pub prev_hash : String,
    pub timestamp : u128,
}

impl BlockTrait for Block {
    fn calculate_hash(b : &Block) -> String {
       let mut hasher = Sha256::new();
       hasher.update(b.prev_hash.as_str());
       hasher.update(format!("{:X}", b.timestamp));
       hasher.update(format!("{:X}", b.nonce));

       return format!("{:X}", hasher.finalize());

    }
}

