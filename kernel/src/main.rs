#![no_std]
#![no_main]

extern crate alloc;

mod arch;
pub mod boot;
pub mod framebuffer;
mod heap;
mod print;

use core::arch::asm;

use framebuffer::FRAMEBUFFER;

#[no_mangle]
extern "C" fn entry() -> ! {
    boot::verify();
    arch::init();

    for (i, c) in "Hello, World!".chars().enumerate() {
        FRAMEBUFFER.lock().draw_char(c, (i, 0));
    }

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
