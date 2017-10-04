#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate crypto_hash;
extern crate serde;
extern crate bincode;

mod node;
mod block;
mod chain;

fn main() {
    // println!("Hello, world!");

    // let mut node = node::Node::new();
    // node.run();

    // Create a blockchain with a starting block
    let mut block = chain::Blockchain::new(vec![0]);
    block.new_block(vec![1, 2, 3, 4]);
    block.print();
}
