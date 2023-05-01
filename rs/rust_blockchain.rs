use crypto::digest::Digest;
use crypto::sha2::Sha256;



struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: i64, data: String, prev_hash: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&mut self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.prev_hash
        );
        hasher.input_str(&input);
        hasher.result_str()
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, 0, "Genesis Block".to_string(), String::from("0"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(
            prev_block.index + 1,
            chrono::Utc::now().timestamp(),
            data,
            prev_block.hash.clone(),
        );
        self.chain.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Transaction 1".to_string());
    blockchain.add_block("Transaction 2".to_string());

    for block in blockchain.chain {
        println!(
            "Block #{} [Hash: {}][Prev Hash: {}]",
            block.index, block.hash, block.prev_hash
        );
    }
}
