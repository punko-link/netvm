#![no_std]
#![no_main]
extern crate alloc;

mod vm_core;
mod libs;

use alloc::string::ToString;
use libs::io::cli;

use core::panic::PanicInfo;



#[no_mangle]
pub extern "C" fn _start() -> ! {
    cli::log("Hallo".to_string());
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cli::log("PANIC!".to_string());
    // let msg = b"Panic!\n";
    // cli::log(1, msg.as_ptr(), msg.len());
    loop {}
}
