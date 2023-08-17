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

// The following code is mostly copied from https://os.phil-opp.com/vga-text-mode/#a-println-macro.

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    DEBUG_CONSOLE.lock().write_fmt(args).unwrap();
}
