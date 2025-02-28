use crate::{error::Result, mutex::Mutex, uart};
use core::fmt::{self, Write};

static mut CONSOLE: Mutex<Console> = Mutex::new(Console {});

struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write(c);
        }
        Ok(())
    }
}
impl Console {
    fn write(&mut self, c: char) -> Result<()> {
        uart::send(c)
    }
}

pub fn _print(args: fmt::Arguments) {
    if let Ok(mut console) = unsafe { CONSOLE.try_lock() } {
        let _ = console.write_fmt(args);
    }
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
