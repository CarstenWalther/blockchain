use std::collections::LinkedList;
use block;

pub struct Blockchain {
    blocks: LinkedList<block::Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut result = Blockchain { blocks: LinkedList::new() };

        result.init_genesis_block();
        result
    }

    fn init_genesis_block(&mut self) {
        self.blocks.push_back(block::Block::new(vec![]))
    }

    pub fn print(&self) {
        for b in &self.blocks {
            b.print();
        }
    }

    fn validate(&self, _block: &block::Block) -> bool {
        // TODO: implement
        true
    }

    pub fn add_block(&mut self, block: block::Block) {
        if self.validate(&block) {
            self.blocks.push_back(block);
        }
    }

    pub fn get_parent_hash(&self) -> [u8; 32] {
        match self.blocks.back() {
            Some(block) => block.hash(),
            None => [0; 32],
        }
    }
}
