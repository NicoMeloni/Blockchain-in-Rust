use std::collections::BTreeMap;

pub struct Pallet{
    block_number: u32,
    nonce: BTreeMap<String, u32>
}

impl Pallet{
    pub fn new() -> Self {
        Pallet{
            block_number: 0,
            nonce: BTreeMap::new()
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    } 

    pub fn inc_block_numer(&mut self){
        self.block_number += 1;
    }

}