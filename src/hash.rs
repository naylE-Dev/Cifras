use sha2::{Sha256, Digest};

pub fn hash(texto: &str) -> Vec<u8> {
    Sha256::digest(texto.as_bytes()).to_vec()
}
