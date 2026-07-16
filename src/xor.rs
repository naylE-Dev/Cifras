
fn xor(texto:&[u8], chave:&[u8]) -> Vec<u8>{
    let mut vet = Vec::new();
    
    for(indice, byte) in texto.iter().enumerate(){//para cada byte numerado
        let pos = indice % chave.len();//pos é igual à ordem(indice) com módulo do tamanho da chave.
        let bit = chave[pos];//aqui pega a letra correspondente ao loop, ou seja, se é o loop 3, vai pegar a terceira letra ou caractere da chave
        let cod = *byte ^ bit;
        vet.push(cod);
    }
    vet
}

fn main(){
    let texto = "Olamundo".as_bytes();
    let chave = "mundo".as_bytes();
    
    let cifra = xor(texto, chave);
    let decifra = xor(&cifra, chave);
    
    let mut cifrado = String::new();
    let decifrado = String::from_utf8(decifra).unwrap();
    
    for byt in &cifra{
        cifrado.push_str(&format!("{:02x}", byt))
    }

    println!("{}", cifrado);
    println!("{}", decifrado);
}
