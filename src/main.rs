#![no_std]
#![no_main]

mod sbi;

use core::arch::global_asm;
use core::ffi::c_int;
use core::panic::PanicInfo;

global_asm!(
    r#"
    .pushsection .text.entry
    .global _start
  _start:
    li sp, 0x80400000
    j main
    .popsection
"#
);

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
