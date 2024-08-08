#![no_std]
#![no_main]
extern crate alloc;

mod vcore;
mod sys_gnu_x86_64;
mod dist;

use alloc::vec::Vec;
use sys_gnu_x86_64::io::cli;
use sys_gnu_x86_64::process::exit;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let a: Vec<u8> = Vec::new();
    cli::log("Hi");

    exit::ok();
}
