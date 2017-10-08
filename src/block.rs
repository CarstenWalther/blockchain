use bincode::serialize;
use bincode::Infinite;
use crypto_hash::{Algorithm, digest};
use serde_bytes;
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct Block {
    index: u32,
    timestamp: u64,
    nonce: u64,
    #[serde(with = "serde_bytes")]
    pub parent: [u8; 32],
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>
}

impl Block
{
    pub fn new(payload: Vec<u8>) -> Block
    {
        
        Block{
            index: 0,
            timestamp: Block::get_current_time(),
            nonce: 0,
            parent: [0;32],
            payload: payload
        }
    }

    fn get_current_time() -> u64 
    {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        current_time.as_secs() * 1000 + current_time.subsec_nanos() as u64 / 1_000_000
    }

    pub fn hash(&self) -> [u8;32]
    {
        let encoded = serialize(&self, Infinite).unwrap();
        let d = digest(Algorithm::SHA256, &encoded);
        let mut arr = [0u8; 32];
        for i in 0..32
        {
            arr[i] = d[i];
        }
        arr
    }

    pub fn proof_of_work(&mut self) {
        self.nonce = 0;
        while &self.hash()[..2] != [0; 2]
        {
            self.nonce += 1;
        }
        println!("Found block at nonce: {}", self.nonce);
    }

    pub fn print(&self)
    {
        let mut s = String::new();
        for byte in self.hash().to_vec()
        {
            write!(&mut s, "{:02X} ", byte).expect("Unable to write");
        }
        println!("Hash: {}", s);
    }
}