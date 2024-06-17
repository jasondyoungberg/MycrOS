use core::{arch::asm, fmt::Write};

use log::{Level, LevelFilter, Log};
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;

use crate::display::DISPLAY;

static LOGGER: Logger = Logger;

struct Logger;

pub fn init() {
    log::set_logger(&LOGGER).expect("This function should only be called once");
    log::set_max_level(LevelFilter::Trace);
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        let color = match record.level() {
            Level::Error => 91,
            Level::Warn => 93,
            Level::Info => 96,
            Level::Debug => 92,
            Level::Trace => 90,
        };

        without_interrupts(|| {
            DEBUGCON
                .lock()
                .write_fmt(format_args!(
                    "\x1b[{}m[{}] {}\x1b[0m\n",
                    color,
                    record.target(),
                    record.args()
                ))
                .expect("Debugcon should never fail");

            DISPLAY
                .lock()
                .write_fmt(format_args!("[{}] {}\n", record.target(), record.args()))
                .expect("Display should never fail");
        });
    }

    fn flush(&self) {
        todo!()
    }
}

static DEBUGCON: Mutex<DebugconWriter> = Mutex::new(DebugconWriter);

struct DebugconWriter;

impl Write for DebugconWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            // Safety: Writing a byte to debugcon should have no side effects
            unsafe { asm!("out 0xe9, al", in("al") b, options(nostack)) };
        }
        Ok(())
    }
}
