#![no_std]
#![no_main]

mod vcore;
mod sys_gnu_x86_64;
mod dist;

use sys_gnu_x86_64::io::cli;
use sys_gnu_x86_64::process::exit;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    cli::log("Hi");

    exit::ok();
}
