use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex, //índice do bloco não condiz com sua posição na blockchain
    InvalidHash, //hash não satisfaz a dificuldade
    IncorrectTimestamp, //momento de criação do bloco ambíguo em relação aos vizinhos
    MismatchedPreviousHash, //hash do bloco anterior não condiz com a sequencia da blockchain
    InvalidGenesisBlock, //checa se o suposto bloco genesis tem a hash anterior composta apenas de zeros
    InvalidInput, //checa se todos os outputs sendo enviados nas transações do novo bloco (inputs) estão disponíveis para envio dentro da blockchain
    InsufficientInputValue, //dinheiro saindo é maior que dinheiro disponível
    InvalidCoinbaseTransaction,

}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash> //UTXOs (Unspent Transaction Outputs)
}

impl Blockchain {

    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new()
        }
    }
    //função que checa se é possível adicionar novo bloco à blockchain
    pub fn update_blockchain(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);

        } else if !block::is_difficulty_greater(&block.hash(), block.difficulty){
            return Err(BlockValidationErr::InvalidHash)

        } else if i != 0 {
            //Outros blocos
            let prev_block = &self.blocks[i-1];
            if block.prev_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);

            } else if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::IncorrectTimestamp); 

            }
        } else {
            //Genesis Block (primeiro bloco)
            if block.prev_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlock)

            }
        }
        

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            // if let resumidamente: se o que vem depois de = puder ser decomposto no que vem depois de let, realizar o then.
            // se block.transactions.(...) pode ser desestruturado em Some((..., ...)), realizar o then.
            // se block.transactions.(...) não retornar nada, else falha com None()
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent_outputs: HashSet<Hash> = HashSet::new();
            let mut block_created_outputs: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            //lidando com as transações
            for transaction in transactions {
                /*
                esse for serve para garantir que todas as transações que estão sendo recebidas e gastas (inputs) no novo bloco fazem parte dos
                outputs ainda não gastos dentro da blockchain como um todo (dinheiro que foi recebido, mas que ainda não foi enviado para ninguém)
                Ou seja, está checando se o dinheiro que está sendo enviado está disponível para uso dentro da blockchain (montante não gasto)
                */
                let input_hashes = transaction.input_hashes();

                //se a quantidade de inputs da transação NÃO estarem na lista de outputs não gastos da blockchain, erro.
                if !(&input_hashes - &self.unspent_outputs).is_empty() || !(&input_hashes & &block_spent_outputs).is_empty(){
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                //se a quantidade de dinheiro saindo (output) da transação for maior que a que entrou (input), erro
                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent_outputs.extend(input_hashes);
                block_created_outputs.extend(transaction.output_hashes());
            }
        
            //lidando com a coinbase (primeira transação)
            if coinbase.output_value() < total_fee { //
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
                //PAREI DAQUI
            } else {
                block_created_outputs.extend(coinbase.output_hashes());
            }

            //remove dos outputs não gastos da blockchain os outputs gastos no bloco sendo analisado
            self.unspent_outputs.retain(|unspent_outputs| !block_spent_outputs.contains(unspent_outputs));
            self.unspent_outputs.extend(block_created_outputs);
        }

        self.blocks.push(block);

        Ok(())
    }
}