use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    pub fn new(index: u32, timestamp: i64, data: String, prev_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
        };

        block.hash = block.calc_hash();
        block
    }

    fn calc_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}",
            self.data, self.index, self.prev_hash, self.timestamp
        );
        hasher.input_str(&input);
        hasher.result_str()
    }
}
#[derive(Debug)]
struct BlockChain {
    chain: Vec<Block>,
}
impl BlockChain {
    pub fn new() -> Self {
        let genesis = Block::new(0,0,"Genesis Block".to_string(),"0".to_string());
        BlockChain {
            chain: vec![genesis],
        }
    }
    pub fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(
            prev_block.index + 1,
            chrono::Utc::now().timestamp(),
            data,
            prev_block.hash.clone(),
        );
        self.chain.push(new_block);    
    }
    pub fn is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i > 0 {
                let prev_block = &self.chain[i - 1];
                if block.prev_hash != prev_block.hash {
                    return false;
                }
            }
            
            if block.hash != block.calc_hash() {
                return false;
            }
        }
        return true;
    }
}
fn main() {
    let mut blockchain = BlockChain::new();
    blockchain.add_block("Some data".to_string());
    blockchain.add_block("More data".to_string());
    println!("Is chain valid? : {}",blockchain.is_chain_valid());
}
