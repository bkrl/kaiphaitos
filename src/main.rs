#![no_std]
#![no_main]

mod io;
mod paging;
mod sbi;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("head.S"), options(raw));

#[no_mangle]
pub extern "C" fn main() -> ! {
    use core::fmt::Write;
    io::DEBUG_CONSOLE
        .lock()
        .write_str("Hello, world!\n")
        .unwrap();
    // Shutdown the system.
    sbi::system_reset(0, 0);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
