use xor_loader_fb0sh::*;
fn main() {
    let shellcode = r2sc();
    let key = g_key(16);

    let encrypted = enc(&key, &shellcode);
    let _ = p_out(&key, &encrypted);

    println!("原始数据");
    println!("{:?}", &key);
    println!("{:?}", &shellcode);

    let encrypted = enc(&key, &shellcode);
    let (bkey, bsc) = p_out(&key, &encrypted);
    let (k, esc) = p_in(&format!("{}:{}", bkey, bsc));
    println!("加密数据");
    println!("{:?}", &k);
    println!("{:?}", &esc);

    let decrypted = dec(&k, &esc);
    println!("解密数据");
    println!("{:?}", &decrypted);
}
