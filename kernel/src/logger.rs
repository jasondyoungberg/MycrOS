use core::{arch::asm, fmt::Write};

use log::{Level, LevelFilter, Log};

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
        DebugconWriter
            .write_fmt(format_args!("[{}] {}\n", record.level(), record.args()))
            .expect("Debugcon should never fail");
    }

    fn flush(&self) {
        todo!()
    }
}

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
