mod balances; //mod é usar para importar]
mod system;
use balances::Pallet;



fn main() {
    let mut pallet_ex: Pallet = Pallet::new();
    pallet_ex.set_balance(&"nicolas".to_string(), 4);

    let balance = pallet_ex.balance(&"nicolas".to_string());

    println!("Balance: {}", balance);
}
