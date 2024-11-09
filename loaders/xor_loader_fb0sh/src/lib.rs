mod obfuscator;
pub use obfuscator::Obfuscator;

mod armagen {
    use super::*;

    pub trait Obfuscation {
        fn new() -> Self;
        fn obfuscate(&mut self, sc: &[u8]);
        fn deobfuscate(&self) -> Vec<u8>;
        fn exec(&self);
    }

    macro_rules! generate_const_struct_code {
        ($struct_name:ident, $const_name:ident, $instance:expr) => {{
            let code = format!(
                "use super::Obfuscator;\nuse once_cell::sync::Lazy;\npub static {}: Lazy<Obfuscator> = Lazy::new(|| \n{:?}\n);",
                stringify!($const_name),
                $instance
            );
            code
        }};
    }

    pub fn g_key(len: usize) -> Vec<u8> {
        let mut key = Vec::new();
        for _ in 0..len {
            key.push(rand::random());
        }
        key
    }

    pub fn gen(obf: &Obfuscator) {
        // 分离或合并代码 这里! 也可以直接为 z.rs
        let code = &generate_const_struct_code!(Obfuscator, O, obf).replace("[", "vec![");
        let cwd = std::env::current_dir().unwrap();
        let out_path = cwd.join("src").join("z.rs");
        std::fs::write(out_path, code).unwrap();
    }

    pub fn load(z_path: &str) -> Obfuscator {
        // 从指定位置加载 待实现
        todo!()
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
}

pub use armagen::*;

mod z;
pub use z::O;
