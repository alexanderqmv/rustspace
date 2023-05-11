use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Block {
    index: i64,
    hash: String,
    prev_hash: String,
    timestamp: i64,
    data: String,
}

fn serialize_block() -> serde_json::Result<()> {
    let block = Block {
        index: 0,
        hash: "".to_owned(), // placeholder for now, we'll calculate the actual hash below
        prev_hash: "0000000000000000000000000000000000000".to_owned(),
        timestamp: chrono::Utc::now().timestamp(),
        data: String::from("Alice sent 2 BTC to Bob"),
    };

    let hash = calculate_hash(&block);
    let block_with_hash = Block { hash: hash, ..block };

    let serialized = serde_json::to_string(&block_with_hash)?;
    println!("{}", serialized);

    Ok(())
}

fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    let input = format!(
        "{}{}{}{}",
        block.index,
        block.prev_hash,
        block.timestamp,
        block.data,
    );
    hasher.input_str(&input);
    hasher.result_str()
}

fn main() {
    let ser = serialize_block();
    if let Err(e) = ser {
        eprintln!("Error serializing block: {}", e);
    }
}
