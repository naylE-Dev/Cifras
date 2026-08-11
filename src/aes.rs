use aes_gcm::{Aes256Gcm, Nonce, Key, KeyInit};
use aes_gcm::aead::Aead;
use aes_gcm::aead::Generate;

pub fn aes(texto: &str, chave: &[u8]) -> (Vec<u8>, Vec<u8>){
    let nonce = Nonce::generate();
    let key = Key::<Aes256Gcm>::from_slice(chave);
    let cifra = Aes256Gcm::new(key);
    let ciphertext = cifra.encrypt(&nonce, texto.as_bytes().as_ref()).unwrap();
    (ciphertext, nonce.to_vec())
}

pub fn decifrar_aes(texto: &[u8], nonce: &[u8], chave: &[u8]) -> String{
    let key = Key::<Aes256Gcm>::from_slice(chave);
    let nonce = Nonce::from_slice(nonce);
    let cifra = Aes256Gcm::new(key);
    
    let ciphertext = cifra.decrypt(nonce, texto.as_ref());
    let stringfinal = String::from_utf8(ciphertext.expect("error")).expect("error");
    stringfinal
    
}
