use super::*;


#[derive(Serialize)]
pub struct BlockchainResponse {
    blocks: Vec<BlockResponse>,
    
}

///UMA ARTIFÍCIO CRIADO APENAS PARA IMPRIMIR OS HASHES DO BLOCO EM FORMATO HEXADECIMAL EM VEZ DE VEC<U8>
#[derive(Serialize)]
pub struct BlockResponse {
    index: u32,
    timestamp: u128,
    prev_hash: String,
    hash: String,
    nonce: u64,
    transactions: Vec<Transaction>, 
}

impl From<&Blockchain> for BlockchainResponse {
    fn from(blockchain: &Blockchain) -> Self {
        BlockchainResponse {
            blocks: blockchain.blocks.iter().map(|block| block.into()).collect(),
            
        }
    }
}

impl From<&Block> for BlockResponse {
    fn from(block: &Block) -> Self {
        BlockResponse {
            index: block.index,
            timestamp: block.timestamp,
            prev_hash: hex::encode(&block.prev_hash), 
            hash: hex::encode(&block.hash), 
            nonce: block.nonce,
            transactions: block.transactions.clone(), 
        }
    }
}
