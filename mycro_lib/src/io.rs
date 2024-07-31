use core::fmt::Write;

use crate::FileDescriptor;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let mut stdout = FileDescriptor::STDOUT;
    write!(stdout, "{}", args).expect("failed to print to stdout");
}

#[doc(hidden)]
pub fn _eprint(args: core::fmt::Arguments) {
    let mut stderr = FileDescriptor::STDERR;
    write!(stderr, "{}", args).expect("failed to print to stderr");
}

/// Prints to the standard output.
/// Equivalent to the `print!` macro in the standard library.
///
/// # Panics
/// Panics if writing to stdout fails.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::print::_print(format_args!($($arg)*));
    }};
}

/// Prints to the standard output, with a newline.
/// Equivalent to the `println!` macro in the standard library.
///
/// # Panics
/// Panics if writing to stdout fails.
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*))
    }};
}

/// Prints to the standard error.
/// Equivalent to the `eprint!` macro in the standard library.
///
/// # Panics
/// Panics if writing to stderr fails.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::print::_eprint(format_args!($($arg)*));
    }};
}

/// Prints to the standard error, with a newline.
/// Equivalent to the `eprintln!` macro in the standard library.
///
/// # Panics
/// Panics if writing to stderr fails.
#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::eprint!("{}\n", format_args!($($arg)*))
    }};
}

/// Prints and returns the value of a given expression for quick and dirty debugging.
/// Equivalent to the `dbg!` macro in the standard library.
///
/// # Panics
/// Panics if writing to stderr fails.
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::eprintln!("[{}:{}:{}]", $crate::file!(), $crate::line!(), $crate::column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}:{}] {} = {:#?}",
                    $crate::file!(), $crate::line!(), $crate::column!(), $crate::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
