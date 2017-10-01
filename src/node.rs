extern crate sha1;

pub struct Node
{
	blockchain: Vec<u64>,
}

impl Node
{
	pub fn new() -> Node
	{
		Node{blockchain: vec![]}
	}

	pub fn run(&mut self)
	{
		println!("Node running!");

		for i in 0u32..10u32
		{
			let mut m = sha1::Sha1::new();
    		m.update(&u32_to_u8_array(i));
    		println!("Hash: {}", m.digest().to_string());
    		self.blockchain.push(i as u64);
		}
	}
}

fn u32_to_u8_array(x:u32) -> [u8;4] {
    let b1 = ((x >> 24) & 0xff) as u8;
    let b2 = ((x >> 16) & 0xff) as u8;
    let b3 = ((x >> 8) & 0xff) as u8;
    let b4 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}