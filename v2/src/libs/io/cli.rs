use alloc::string::String;
use core::arch::asm;
use core::assert_eq;

pub fn log(s: String) {
    let slice = s.as_str();
    let buf = slice.as_ptr();
    let count = slice.len();
    unsafe {
        let ret: usize;

        asm!(
        "syscall",
        in("rax") 1,
        in("rdi") 1,
        in("rsi") buf,
        in("rdx") count,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        );

        assert_eq!(ret, count);
    }
}
