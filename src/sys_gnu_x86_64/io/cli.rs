use core::arch::asm;
use core::assert_eq;

pub fn log(s: &str) {
    let buf = s.as_ptr();
    let count = s.len();
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
