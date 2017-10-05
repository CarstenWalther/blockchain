use chain;

pub struct Node
{
    blockchain: chain::Blockchain,
}

impl Node
{
    pub fn new() -> Node
    {
        Node{blockchain: chain::Blockchain::new(vec![0])}
    }

    pub fn run(&mut self)
    {
        println!("Node running!");

        for _ in 0..10
        {
            self.blockchain.new_block(vec![1, 2, 3, 4]);
        }
        self.blockchain.print();
    }
}
