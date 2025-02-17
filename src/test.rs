use super::*;

#[test]
fn test_blockchain(){
    let difficulty = 0x003fffffffffffffffffffffffffffff;

    let mut blockchain = Blockchain::new();

    /////////////////////////////////////////////////////////////////////////////////
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            from: None,
            to: "Nicolas".to_owned(),
            amount: 400
        },
        Transaction {
            from: Some("Nicolas".to_owned()),
            to: "Nino".to_owned(),
            amount: 150
        },
        Transaction {
            from: Some("Nicolas".to_owned()),
            to: "Lucas".to_owned(),
            amount: 50
        },
    ], difficulty);
    
    blockchain.mine(&mut genesis_block); //minera o bloco para que passe da dificuldade

    println!("GENESIS BLOCK: {:#?}", genesis_block);
    let last_hash = genesis_block.hash.clone();
    blockchain.update_blockchain(genesis_block, "Nicolas".to_owned()).expect("Error adding genesis block");

    
    ////////////////////////////////////////////
    let mut block_1 = Block::new(1, now(), last_hash, vec![
        Transaction {
            from: None,
            to: "Macaco".to_owned(),
            amount: 5000
        },
        Transaction {
            from: Some("Macaco".to_owned()),
            to: "Nino".to_owned(),
            amount: 1000
        },
        Transaction {
            from: Some("Nicolas".to_owned()),
            to: "Lucas".to_owned(),
            amount: 100
        },
    ], difficulty);

    blockchain.mine(&mut block_1); //minera o bloco para que passe da dificuldade

    println!("BLOCK 1: {:#?}", block_1);
    blockchain.update_blockchain(block_1, "Macaco".to_owned()).expect("Error adding block 1");

    println!("LEDGER: {:#?}", blockchain.ledger);
    println!("NICOLAS BALANCE: {:?}", blockchain.get_balance("Nicolas".to_owned()));
    println!("LUCAS BALANCE: {:?}", blockchain.get_balance("Lucas".to_owned()));
    println!("MACACO BALANCE: {:?}", blockchain.get_balance("Macaco".to_owned()));
    println!("BLOCKCHAIN: {:#?}", blockchain);

}