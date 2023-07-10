use core::arch::asm;
use core::ffi::{c_int, c_long};

const EID_CONSOLE_PUTCHAR: i32 = 0x01;

pub fn console_putchar(ch: c_int) -> c_long {
    let ret;
    unsafe {
        asm!("ecall", in("a7") EID_CONSOLE_PUTCHAR, in("a0") ch, lateout("a0") ret);
    }
    ret
}
