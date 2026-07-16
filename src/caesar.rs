pub fn caesar(texto: &str, chave: u8) -> String{
    let mut cod = Vec::new();//vetor que armazena os bytes

    for c in texto.bytes(){//para cada byte(caractere) no texto:
        if c.is_ascii_alphabetic(){//se for ascii:
            let pos = c - b'a';//pega a pos das letras no alfabeto com base0
            let postwo = (pos + chave) % 26;//posiciona os bytes com deslocamento da chave
            let bytefinal = postwo + b'a';//soma com os bytes de a para pegar os bytes reais finais.
            
            cod.push(bytefinal);//manda o resultado pra dentro do vetor            
        }else{//se não for ascii, cai aqui e armazena o byte puro, sem codificar.
            cod.push(c)//manda o byte pro vetor, sem codificar
        }
    }
    let result = String::from_utf8(cod).unwrap();//Transforma todos os números do vetor em letras
    result.trim().to_string()//retira caracteres desnecessários e transforma em uma string
}
