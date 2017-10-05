use bincode::serialize;
use bincode::Infinite;
use crypto_hash::{Algorithm, digest};
use serde_bytes;
use std::fmt::Write;

#[derive(Serialize)]
pub struct Block {
    index: u32,
    timestamp: u64,
    proof: u64,
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
            timestamp: 0,
            proof: 0,
            parent: [0;32],
            payload: payload
        }
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
        self.proof = 0;
        while &self.hash()[..2] != [0; 2]
        {
            self.proof += 1;
        }
    }

    pub fn print(&self)
    {
        let mut s = String::new();
        for byte in self.hash().to_vec()
        {
            write!(&mut s, "{:X} ", byte).expect("Unable to write");
        }
        println!("Hash: {}", s);
    }
}