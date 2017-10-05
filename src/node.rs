use block;
use chain;

pub struct Node
{
    blockchain: chain::Blockchain,
}

impl Node
{
    pub fn new() -> Node
    {
        Node{blockchain: chain::Blockchain::new()}
    }

    pub fn mine(&mut self)
    {
        println!("Start mining");

        for _ in 0..5
        {
            self.mine_block();
        }
        self.blockchain.print();
    }

    fn mine_block(&mut self) {
        let payload = vec![1, 2, 3, 4];
        let mut new_block = block::Block::new(payload);
        new_block.parent = self.blockchain.get_parent_hash();
        
        new_block.proof_of_work();
        self.blockchain.add_block(new_block);
    }
}
