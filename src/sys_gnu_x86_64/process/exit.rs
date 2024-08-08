use core::arch::asm;
use core::panic::PanicInfo;
use crate::sys_gnu_x86_64::io::cli;

pub fn ok() -> ! {
    unsafe {
        asm!(
        "mov eax, 1",
        "mov ebx, 0",
        "int 0x80"
        );
    }

    loop {};
}


pub fn control_panic(msg: &str) -> ! {
    cli::log(msg);
    panic!();
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
        "mov eax, 1",
        "mov ebx, 1",
        "int 0x80"
        );
    }

    loop {};
}
