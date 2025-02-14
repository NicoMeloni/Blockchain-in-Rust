
use super::*;

#[derive(Debug)]
pub struct Block{
    pub index: u32,
    pub timestamp: u128,
    pub prev_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128
    
}

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_hash: Vec<u8>, transactions: Vec<Transaction>, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            prev_hash,
            hash: vec![0, 32], //32 bytes armazenados no vetor, 256 bits
            nonce: 0, //nonce é um valor arbitrário que adicionamos no bloco para que possamos alterar a hash sem ter que alterar
                   //as informações importantes do bloco. O objetivo disso é permitir a mineração para que, eventualmente, 
                   //encontremos um hash que permita a validação desse bloco e a inclusão do mesmo na blockchain. 
            transactions,
            difficulty
        }
    }

    pub fn mine(&mut self) {
        for nonce_num in 0..(u64::MAX){
            self.nonce = nonce_num;
            let hash = self.hash();
            if is_difficulty_greater(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    } 

    
}

impl Hashable for Block { //adiciona todas as características do bloco dentro de um vector de bytes
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.index.to_le_bytes());
        bytes.extend(&self.timestamp.to_le_bytes());
        bytes.extend(&self.prev_hash);
        bytes.extend(&self.nonce.to_le_bytes());
        bytes.extend(self.transactions
                                .iter()
                                .flat_map(|transaction| transaction.bytes())
                            );
                               
        bytes.extend(&self.difficulty.to_le_bytes());

        bytes
    }
}

pub fn is_difficulty_greater(hash: &Vec<u8>, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash).unwrap_or(0)
}

