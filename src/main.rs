#![no_std]
#![no_main]

mod io;
mod memory;
mod sbi;
mod static_assert;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("head.S"), options(raw));

#[no_mangle]
pub extern "C" fn main() -> ! {
    println!("Hello, world!");
    shutdown();
}

fn shutdown() -> ! {
    sbi::system_reset(0, 0);
    panic!("shutdown failed");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
