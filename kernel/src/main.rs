#![no_std]
#![no_main]

extern crate alloc;

mod arch;
pub mod boot;
mod heap;
mod print;

use core::arch::asm;

#[no_mangle]
extern "C" fn entry() -> ! {
    boot::verify();
    arch::init();
    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
