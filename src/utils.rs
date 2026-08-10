pub fn para_hex(hash: &[u8]) -> String{
    let mut fnal = String::new();
    for byt in &hash{
        fnal.push_str(&format!("{:02x}", byt))
    }
    fnal
}
