#[derive(Debug)]
pub struct Block{
    index: u32,
    timestamp: u64,
    prev_hash: Vec<u8>,
    hash: Vec<u8>,
    nonce: u64,
    payload: String
    
}

impl Block {
    pub fn new(index: u32, timestamp: u64, prev_hash: Vec<u8>, nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            prev_hash,
            hash: vec![0, 32], //32 bytes armazenados no vetor, 256 bits
            nonce,
            payload
        }
    }
}