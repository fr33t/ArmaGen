use armagen::*;

fn main() {
    let shellcode = r2sc();

    let mut obf = Obfuscator::new();

    obf.obfuscate(&shellcode);

    gen(&obf);
}
