use blockchainlib::*;



fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![
                transaction::Output {
                    to_address: "Crhis".to_owned(),
                    value: 50
                },
            ],
            outputs: vec![
                

                transaction::Output {
                    to_address: "Matheus".to_owned(),
                    value: 40
                },

                transaction::Output {
                    to_address: "Lucas".to_owned(),
                    value: 40
                }
            ]
        }
    ], difficulty);

    

    genesis_block.mine(); //minera o bloco para que passe da dificuldade

    println!("GENESIS BLOCK: {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_blockchain(genesis_block).expect("Error adding genesis block");

    
}
