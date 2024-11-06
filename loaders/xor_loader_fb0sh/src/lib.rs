mod armagen;
pub use armagen::{g_key, p_in, p_out, r2sc};

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

pub static ksc: &'static str = "xWJEpLAraVYW2nLsMEjyqw==:OSrHQEDDqVYW2jO9cRig+pMqdXbVY+IEdpL5vigAefnlKs/W4GNm4VyQP935AMNraV4l2LIHSRfXE3+tMYkQRpcjFew7eUndVOY67eDDciPFYkTsNesdMV7bory7AOrvTiJk7bH7igBeJbutu3x648S0CZV5Y1iWupuzJT0J82r9gjFV/Cglch6fSz1FkKrvTiJg7bH7Dxed1jqouwju4sSyBS+0oyFXxpsqrWgWq/GEOgX98XEh1fr6M77PqKrqnDgML6LCPqnpJS+kiknyq8ViRKSwY+TbF9ty7HHywyCq5btxC9vc9ECbyEql9W9UECrHYJgXbyocWokMRU1J7NYQK86wcijfzCWnj1EkkYWgGiGk";
