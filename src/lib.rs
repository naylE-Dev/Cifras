use wasm_bindgen::prelude::*;

mod caesar;
mod xor;
mod hash;
mod aes;
mod utils;

use caesar::caesar;
use xor::xor;
use hash::hash;
use utils::para_hex;
use aes::aes;
use aes::decifrar_aes;

#[wasm_bindgen]
pub fn caesar_wasm(texto: &str, chave: u8) -> String {
    caesar(texto, chave)
}

#[wasm_bindgen]
pub fn xor_wasm(texto: &[u8], chave:  &[u8]) -> Vec<u8> {
    xor(texto, chave)
}

#[wasm_bindgen]
pub fn hash_wasm(texto: &str) -> Vec<u8> {
    hash(texto)
}

#[wasm_bindgen]
pub struct AesResultado {
    ciphertext: Vec<u8>,
    nonce: Vec<u8>,
}

#[wasm_bindgen]
impl AesResultado {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}

#[wasm_bindgen]
pub fn aes_wasm(texto: &str, chave: &[u8]) -> AesResultado {
    let (ciphertext, nonce) = aes(texto, chave);
    AesResultado{ ciphertext, nonce }
}

#[wasm_bindgen]
pub fn decifrar_aes_wasm(texto: &[u8], nonce: &[u8], chave: &[u8]) -> String {
    decifrar_aes(texto, nonce, chave)
}
