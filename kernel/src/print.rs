use core::fmt::{self, Write};

use spin::Mutex;

use crate::arch;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::println!("[{}:{}:{}] {} = {:#?}",
                    file!(), line!(), column!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

struct Console;
static CONSOLE: Mutex<Console> = Mutex::new(Console);
impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        arch::debug_print(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    CONSOLE.lock().write_fmt(args).unwrap()
}
