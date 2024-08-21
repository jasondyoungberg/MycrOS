use core::{
    arch::asm,
    fmt::{self, Write},
};

use spin::mutex::Mutex;

static CONSOLE: Mutex<DebugCon> = Mutex::new(DebugCon);

pub struct DebugCon;

impl Write for DebugCon {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            asm!(
                "rep outs dx, byte ptr [rsi]",
                in("dx") 0xe9,
                in("rsi") s.as_ptr(),
                in("rcx") s.len(),
            )
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    CONSOLE.lock().write_fmt(args).unwrap()
}
