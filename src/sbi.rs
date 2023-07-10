use core::arch::asm;
use core::ffi::{c_int, c_long};

const EID_CONSOLE_PUTCHAR: i32 = 0x01;
const EID_SYSTEM_RESET: i32 = 0x53525354;

pub struct Sbiret {
    pub error: c_long,
    pub value: c_long,
}

pub fn console_putchar(ch: c_int) -> c_long {
    let ret;
    unsafe {
        asm!("ecall", in("a7") EID_CONSOLE_PUTCHAR, in("a0") ch, lateout("a0") ret);
    }
    ret
}

pub fn system_reset(reset_type: u32, reset_reason: u32) -> Sbiret {
    let error;
    let value;
    unsafe {
        asm!("ecall",
        in("a7") EID_SYSTEM_RESET, in("a6") 0, in("a0") reset_type, in("a1") reset_reason,
        lateout("a0") error, lateout("a1") value
        );
    }
    Sbiret { error, value }
}
