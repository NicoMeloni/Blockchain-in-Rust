use std::collections::BTreeMap; //estrutura de dados com chave e valor (Dicionário, quase)

//Pallet se refere a cada "componente" da nossa blockchain

//Balance é o componente responsável por verificar e transferir quantias entre usuários
pub struct Pallet{
    balances : BTreeMap<String, u128>
} 

impl Pallet{
    pub fn new() -> Self{
        Pallet {
            balances: BTreeMap::new() 
        }
    }

    pub fn set_balance(&mut self, account: &String, amount: u128){
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&self, account: &String) -> u128 {
        *self.balances.get(account).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: String, //pessoa que está transferindo
        to: String, //pessoa que está recebendo a transferência
        amount: u128 //quantidade a tentar transferir
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[test]

fn init_balances(){ //um único teste para testar tudo
    let mut balances = Pallet::new();

    assert_eq!(balances.balance(&"nicolas".to_string()), 0);

    balances.set_balance(&"nicolas".to_string(), 10);

    assert_eq!(balances.balance(&"nicolas".to_string()), 10);
}

#[test]

fn transfer_balance(){
    let mut balances = Pallet::new();

    balances.set_balance(&"nicolas".to_string(), 10);
    balances.set_balance(&"joao".to_string(), 6);

    assert_eq!(balances.transfer("nicolas".to_string(), "joao".to_string(), 10), Ok(()));

    assert_eq!(balances.balance(&"nicolas".to_string()), 0);

    assert_eq!(balances.transfer("joao".to_string(), "nicolas".to_string(), 17),Err("Insufficient balance"));

    assert_eq!(balances.transfer("joao".to_string(), "nicolas".to_string(), 7), Ok(()));

    assert_eq!(balances.balance(&"nicolas".to_string()), 7);
    assert_eq!(balances.balance(&"joao".to_string()), 9);
}