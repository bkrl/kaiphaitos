use core::ffi::c_int;
use core::fmt;

use spin::Mutex;

use crate::sbi;

pub struct DebugConsole {}

impl fmt::Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            if sbi::console_putchar(*byte as c_int) != 0 {
                return Err(fmt::Error {});
            }
        }
        Ok(())
    }
}

pub static DEBUG_CONSOLE: Mutex<DebugConsole> = Mutex::new(DebugConsole {});
