use std::collections::LinkedList;
use block;

pub struct Blockchain {
    blocks: LinkedList<block::Block>
}

impl Blockchain {

    pub fn new() -> Blockchain {
        let mut result = Blockchain {
            blocks: LinkedList::new()
        };

        result.init_genesis_block();
        result
    }

    fn init_genesis_block(&mut self)
    {
        self.blocks.push_back(block::Block::new(vec![]))
    }

    pub fn print(&self)
    {
        for b in &self.blocks
        {
            b.print();
        }
    }

    pub fn new_block(&mut self, payload: Vec<u8>) {
        let mut new_block = block::Block::new(payload);
        new_block.parent = match self.blocks.back() {
            Some(x) => x.hash(),
            None => [0; 32]
        };
        
        new_block.proof_of_work();
        self.blocks.push_back(new_block);
    }
}