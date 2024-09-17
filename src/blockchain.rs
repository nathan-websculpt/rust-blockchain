use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

//verification of block
//not nearly close to a real-world setting, just for testing
impl Blockchain {
    pub fn verify(&self) -> bool {
        //if block not located where it claims to be located -- fails test
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index invalid: {} != {}", &block.index, &i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                //in a real-world setting, the network would provide difficulty
                println!("Hash invalid");
                return false;
            }
        }
        true
    }
}
