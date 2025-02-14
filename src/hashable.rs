//colisão: quando 2 dados de entrada distintos geram o exato mesmo hash
use super::*;
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Hash { //função que cria o hash do bloco com base no algoritmo SHA256 usando as informações do bloco transformadas em bytes
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}