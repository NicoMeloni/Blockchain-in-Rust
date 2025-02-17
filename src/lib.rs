use std::time::{SystemTime, UNIX_EPOCH};
pub use serde::{Deserialize, Serialize};

mod block;
pub use crate::block::Block;
mod hashable;
pub use crate::hashable::Hashable;
mod blockchain;
pub use crate::blockchain::Blockchain;
pub mod transaction;
pub use crate::transaction::Transaction;
mod test;
mod blockchain_json;
pub use crate::blockchain_json::{BlockchainResponse, BlockResponse};

type Hash = Vec<u8>;
type Address = String;

pub fn saudacao() {
    println!("Olá, Rustacean!");
}

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn difficulty_bytes_as_u128(v: &Vec<u8>) -> Option<u128> {
    match v[16..32].try_into() {
        Ok(array) => Some(u128::from_be_bytes(array)),
        Err(_) => None,
    }
}

pub fn is_difficulty_greater(hash: &Vec<u8>, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash).unwrap_or(0)
}

pub fn option_string_to_bytes(opt: Option<String>) -> Vec<u8> {
    match opt {
        Some(s) => s.into_bytes(), // Transforma a String em bytes
        None => Vec::new(), // Retorna um vetor vazio se for None
    }
}


