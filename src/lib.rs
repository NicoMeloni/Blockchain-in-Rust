use std::time::{SystemTime, UNIX_EPOCH};

mod balances; //mod é usar para importar]
mod system;
mod block;
pub use crate::block::Block;
mod hashable;
pub use crate::hashable::Hashable;
mod blockchain;
pub use crate::blockchain::Blockchain;
pub mod transaction;
pub use crate::transaction::Transaction;

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


