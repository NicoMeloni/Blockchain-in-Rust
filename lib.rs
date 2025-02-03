use std::time::{SystemTime, UNIX_EPOCH};
mod balances; //mod é usar para importar]
mod system;
mod block;
pub use crate::block::Block;

pub fn saudacao() {
    println!("Olá, Rustacean!");
}

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

