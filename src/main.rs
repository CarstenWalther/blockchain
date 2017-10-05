#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate crypto_hash;
extern crate serde;
extern crate bincode;

mod block;
mod node;
mod chain;

fn main() {
    let mut node = node::Node::new();
    node.mine();
}
