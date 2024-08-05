use core::arch::asm;
use core::panic::PanicInfo;

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
