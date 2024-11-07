mod armagen;
mod sc;
pub use armagen::{g_key, p_in, p_out, r2sc};
pub use sc::KSC;

pub fn enc(key: &[u8], sc: &[u8]) -> Vec<u8> {
    // xor 加密
    let mut enc_sc = Vec::new();
    for (i, byte) in sc.iter().enumerate() {
        let key_byte = key[i % key.len()];
        let enc_byte = *byte ^ key_byte;
        enc_sc.push(enc_byte);
    }
    enc_sc
}

pub fn dec(key: &[u8], esc: &[u8]) -> Vec<u8> {
    // xor 解密
    let mut dec_esc = Vec::new();
    for (i, byte) in esc.iter().enumerate() {
        let key_byte = key[i % key.len()];
        let dec_byte = *byte ^ key_byte;
        dec_esc.push(dec_byte);
    }
    dec_esc
}
