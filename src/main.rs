use blockchainlib::*;


use axum::{
    extract::State, http::StatusCode, response::{Html, IntoResponse}, routing::{delete, get, post}, Json, Router
};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;

const DIFFICULTY: u128 = 0x0005ffffffffffffffffffffffffffff;

///ADICIONA A TRANSAÇÃO AO BLOCO ATUAL. NÃO CHECA SE A TRANSAÇÃO É VÁLIDA, ISSO É FEITO APENAS NA HORA DE ADICIONAR O BLOCO À BLOCKCHAIN
async fn process_transaction(
    State((_blockchain, block_state)): State<(Arc<Mutex<Blockchain>>, Arc<Mutex<Block>>)>,
    Json(payload): Json<Transaction>,
) -> impl IntoResponse {
    let mut block = block_state.lock().await;

    block.transactions.push(
        Transaction::new(
            payload.from.clone(),
            payload.to.clone(),
            payload.amount
        )
    );

    (StatusCode::OK, Json(json!({"status": "success", "message": "Transação adicionada ao bloco atual"})))    
}

///AQUI O BLOCO VAI SER MINERADO E ADICIONADO À BLOCKCHAIN SE TUDO CORRER BEM
async fn mine_block(
    State((blockchain, block_state)): State<(Arc<Mutex<Blockchain>>, Arc<Mutex<Block>>)>,
    Json(miner) : Json<String>,
) -> impl IntoResponse {
    let mut block = block_state.lock().await;
    let mut blockchain = blockchain.lock().await;

    // Minera o bloco e obtém o hash
    let hash = blockchain.mine(&mut *block);
    
    // Adiciona o bloco minerado à blockchain e, se tudo der certo, recompensa o minerador.
    // Para esse projeto disciplinar, o mínimo de transações por bloco é 3 para que ele possa ser minerado.
    if block.transactions.len() < 3  {
        return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"status": "error",
                                                             "message": "O bloco precisa de pelo menos 3 transações para ser minerado"
                                                             })));

    } else if let Err(e) = blockchain.update_blockchain(block.clone(), miner.clone()) {
        return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"status": "error",
                                                             "message": format!("Erro ao atualizar blockchain: {:?}", e)
                                                             })));
    } else {
        // Se a atualização for bem-sucedida, cria um novo bloco para o próximo ciclo
    *block = Block::new(blockchain.blocks.len() as u32, now(), hash.clone(), vec![], DIFFICULTY);

    //retorno
    (StatusCode::OK, Json(json!({"status": "success", 
                                   "hash": hex::encode(hash)
                                })))
    }
}

///FUNÇÃO PARA PEGAR O ESTADO ATUAL DA BLOCKCHAIN
async fn get_blockchain(
    State((blockchain_state, _block_state)): State<(Arc<Mutex<Blockchain>>, Arc<Mutex<Block>>)>,
) -> impl IntoResponse {
    let blockchain = blockchain_state.lock().await;
    let response = BlockchainResponse::from(&*blockchain);
    (StatusCode::OK, Json(response))
}

///FUNÇÃO PARA PEGAR O BLOCO ATUAL EM QUE AS TRANSAÇÕES ESTÃO SENDO ADICIONADAS
async fn get_block(
    State((_blockchain_state, block_state)): State<(Arc<Mutex<Blockchain>>, Arc<Mutex<Block>>)>,
) -> impl IntoResponse {
    let block = block_state.lock().await;
    let response = BlockResponse::from(&*block);
    (StatusCode::OK, Json(response))
}

///FUNÇÃO PARA PEGAR A LISTA DE USUÁRIOS E SEUS RESPECTIVOS SALDOS
async fn get_ledger(
    State((blockchain_state, _block_state)): State<(Arc<Mutex<Blockchain>>, Arc<Mutex<Block>>)>,
) -> impl IntoResponse {
    let blockchain = blockchain_state.lock().await;
    let response = blockchain.ledger.clone();
    (StatusCode::OK, Json(response))
}

///FUNÇÃO PARA APAGAR O BLOCO ATUAL EM CASO DE ERRO
async fn clear_current_block(
    State((_blockchain_state, block_state)): State<(Arc<Mutex<Blockchain>>, Arc<Mutex<Block>>)>,
) -> impl IntoResponse {
    let mut block = block_state.lock().await;
    *block = Block::new(block.index, now(), block.prev_hash.clone(), vec![], DIFFICULTY);
    (StatusCode::OK, Json(json!({"status": "success", 
                                    "message": "Bloco apagado com sucesso"
                                })))
}

///FUNÇÃO PARA REMOVER A ÚLTIMA TRANSAÇÃO ADICIONADA NO BLOCO ATUAL
async fn delete_transaction(
    State((_blockchain_state, block_state)): State<(Arc<Mutex<Blockchain>>, Arc<Mutex<Block>>)>,
) -> impl IntoResponse {
    let mut block = block_state.lock().await;
    if let Some(removed) = block.transactions.pop() {
        (StatusCode::OK, Json(json!({"status": "success", "removed_transaction": removed})))
    } else {
        (StatusCode::BAD_REQUEST, Json(json!({"status": "error", "message": "Não há transações para remover"})))
    }
}

#[tokio::main]
async fn main() {

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let current_block = Arc::new(Mutex::new(Block::new(0, now(), vec![0; 32], vec![], DIFFICULTY)));


    let app = Router::new()
        .route("/transaction", post(process_transaction))
        .route("/mine", post(mine_block))
        .route("/blockchain", get(get_blockchain))
        .route("/block", get(get_block))
        .route("/ledger", get(get_ledger))
        .route("/clear_block", delete(clear_current_block))
        .route("/delete_transaction", delete(delete_transaction))
        .route("/", get(|| async { Html("Welcome to Blockchain!")}))
        .with_state((blockchain.clone(), current_block.clone()));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

