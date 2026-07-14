use std::io;

fn caesar(texto: &str, chave: u8) -> String{
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

fn main(){
    loop{
        println!("Você deseja Cifrar ou DEcifrar? \n Digite 1 para Cifrar e 2 para DEcifrar\n Digite x para sair\n");
        let mut esclha = String::new();
        io::stdin().read_line(&mut esclha).expect("falha ao ler linha\n");
        let escolha = esclha.trim();

        if !matches!(escolha, "1" | "2" | "x"){
            println!("Digite APENAS 1, 2 OU x(minúsculo).");
            continue;
        }else if escolha == "x"{
            break;
        }
    
        println!("Digite o que você deseja cifrar/decifrar\n");
        let mut txt = String::new();//cria nova string vazia
        io::stdin().read_line(&mut txt).expect("Falha ao ler a linha\n");//lê input do usuário e guarda em txt
        let text = txt.to_ascii_lowercase();//transforma o input do usuário em ascii e lowercase

        println!("Digite quantas casas você deseja andar/voltar no alfabeto\n");
        let mut cve = String::new();
        io::stdin().read_line(&mut cve).expect("Falha ao ler a linha\n");
        let chv: u8 = cve.trim().parse().expect("Digite um número inteiro válido\n");
        let chve = chv % 26;
        
        if escolha == "1"{
            let cifra = caesar(&text, chve);
            println!("Seu texto cifrado é: {}\n", cifra)

        }else if escolha == "2"{
            let cifra = caesar(&text, 26 - chve);
            println!("Seu texto decifrado é: {}\n", cifra);

        }
    }
}
