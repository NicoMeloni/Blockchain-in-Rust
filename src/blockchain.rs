pub use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex, //índice do bloco não condiz com sua posição na blockchain
    InvalidHash, //hash não satisfaz a dificuldade
    IncorrectTimestamp, //momento de criação do bloco ambíguo em relação aos vizinhos
    MismatchedPreviousHash, //hash do bloco anterior não condiz com a sequencia da blockchain
    InvalidGenesisBlock, //checa se o suposto bloco genesis tem a hash anterior composta apenas de zeros
    InvalidCoinbaseTransaction,
    NotEnoughBalance
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub ledger: HashMap<Address, u64>,
    pub block_reward: u64 //HashMap com endereço e outputs relacionados a esse endereço (contas e saldos)
}

impl Blockchain {

    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            ledger: HashMap::new(),
            block_reward: 50
        }
    }
    
    ///SE O BLOCO FOR VÁLIDO, SERÁ ADICIONADO À BLOCKCHAIN E O LEDGER SERÁ ATUALIZADO COM update_ledger()
    pub fn update_blockchain(&mut self, block: Block, miner: Address) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        //////////////////TESTE DE ÍNDICE////////////////
        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);

        //////////////////TESTE DE DIFICULDADE////////////////
        } else if !is_difficulty_greater(&block.hash(), block.difficulty){
            return Err(BlockValidationErr::InvalidHash)

        //////////////////NÃO É O GENESIS BLOCK?////////////////
        } else if i != 0 {
            let prev_block = &self.blocks[i-1];
            if block.prev_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);

            } else if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::IncorrectTimestamp); 

            }
        //////////////////É O GENESIS BLOCK!!////////////////
        } else {
            //Genesis Block (primeiro bloco)
            if block.prev_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlock)

            }
        }

        let mut coinbases = 0;
        //////////////////TRANSAÇÕES////////////////
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            // if let resumidamente: se o que vem depois de = puder ser decomposto no que vem depois de let, realizar o then.
            // se block.transactions.(...) pode ser desestruturado em Some((..., ...)), realizar o then.
            // se block.transactions.(...) não retornar nada, else falha com None()
            
            if coinbase.is_coinbase() && coinbases ==0 {
                self.update_ledger(coinbase.from.clone(), coinbase.to.clone(), coinbase.amount.clone());
                coinbases += 1;

            } else {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }            

            //lidando com as transações
            for transaction in transactions {
                //TENTA FAZER A TRANSAÇÃO, SE FALHAR RETORNA ERRO
                if transaction.from == None {
                    return Err(BlockValidationErr::InvalidCoinbaseTransaction);
                } else {
                    if !self.update_ledger(transaction.from.clone(), transaction.to.clone(), transaction.amount.clone()) {
                        return Err(BlockValidationErr::NotEnoughBalance);
                    } 
                }
                 
            }           
        }
            self.update_ledger(None, miner, self.block_reward);
            self.blocks.push(block);
            Ok(())

    }

    pub fn get_balance(&self, address: String) -> u64 {
       return self.ledger[&address];
    }

    ///ATUALIZA O LEDGER CONFORME TRANSAÇÕES DO BLOCO APROVADO
    pub fn update_ledger(&mut self, from: Option<Address>, to: Address, amount: u64,) -> bool {
        // Se for uma transação normal (não coinbase), verificamos saldo
        if let Some(f) = from {
            //verificar se o remetente tem saldo suficiente
            let sender_balance = self.ledger.get(&f).copied().unwrap_or(0);
            if sender_balance < amount {
                return false; 
            }

            // subtrair o valor da conta do remetente
            self.ledger.insert(f.clone(), sender_balance - amount);
        // se for coinbase...
        } 

        //adicionar o valor à conta do destinatário
        let recipient_balance = self.ledger.get(&to).copied().unwrap_or(0);
        self.ledger.insert(to, recipient_balance + amount);

        true // Transação bem-sucedida
    }

    ///MINERA O BLOCO PARA ENCONTRAR UMA HASH QUE SATISFAÇA A DIFICULDADE
    pub fn mine(&mut self, block: &mut Block) -> Hash {
        let mut real_hash = vec![];
        for nonce_num in 0..(u64::MAX){
            block.nonce = nonce_num;
            let hash = block.hash();
            if is_difficulty_greater(&hash, block.difficulty) {
                block.hash = hash.clone();
                real_hash = hash.clone();
                break;
            }
        }
        return real_hash;
    }

}