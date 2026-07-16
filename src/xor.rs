pub fn xor(texto:&[u8], chave:&[u8]) -> Vec<u8>{
    let mut vet = Vec::new();
    
    for(indice, byte) in texto.iter().enumerate(){//para cada byte numerado
        let pos = indice % chave.len();//pos é igual à ordem(indice) com módulo do tamanho da chave.
        let bit = chave[pos];//aqui pega a letra correspondente ao loop, ou seja, se é o loop 3, vai pegar a terceira letra ou caractere da chave
        let cod = *byte ^ bit;
        vet.push(cod);
    }
    vet
}

