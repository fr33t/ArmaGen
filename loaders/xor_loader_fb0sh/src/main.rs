use xor_loader_fb0sh::*;
fn main() {
    let shellcode = r2sc();
    let key = g_key(16);

    let encrypted = enc(&key, &shellcode);
    let _ = p_out(&key, &encrypted);
}
