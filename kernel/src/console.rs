use crate::{framebuffer_console, mutex::Mutex, uart};
use core::fmt::{self, Write};

static mut CONSOLE: Mutex<Console> = Mutex::new(Console);
static mut DEBUG_CONSOLE: DebugConsole = DebugConsole;

struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = uart::puts(s);
        Ok(())
    }
}

struct DebugConsole;

impl fmt::Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        uart::debug_puts(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    let _ = unsafe { DEBUG_CONSOLE.write_fmt(args) };

    if let Ok(mut console) = unsafe { CONSOLE.try_lock() } {
        let _ = console.write_fmt(args);
    }

    let _ = framebuffer_console::write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
