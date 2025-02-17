use crate::Hashable;
use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from: Option<Address>,  // Quem está enviando
    pub to: Address,    // Quem recebe
    pub amount: u64,    // Quantia transferida
}


impl Transaction {

    pub fn new(from: Option<Address>, to: Address, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.from == None
    }
 }



impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(option_string_to_bytes(self.from.clone()));
        bytes.extend(self.to.as_bytes());

        bytes.extend(&self.amount.to_le_bytes());   
        
        bytes
    }
}