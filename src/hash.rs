pub fn hash(texto: &str) -> String{
    let mut fnal = String::new();
    let hsh = Sha256::digest(texto.as_bytes());
    for byt in &hsh{
        fnal.push_str(&format!("{:02x}", byt))
    }
    fnal
}
