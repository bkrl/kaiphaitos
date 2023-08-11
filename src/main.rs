#![no_std]
#![no_main]

mod paging;
mod sbi;

use core::arch::global_asm;
use core::ffi::c_int;
use core::panic::PanicInfo;

global_asm!(include_str!("head.S"), options(raw));

#[no_mangle]
pub extern "C" fn main() -> ! {
    for c in "Hello, world!\n".as_bytes() {
        sbi::console_putchar(*c as c_int);
    }
    sbi::system_reset(0, 0);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
