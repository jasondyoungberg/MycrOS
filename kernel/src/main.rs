#![no_std]
#![no_main]
#![allow(dead_code)]

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

    println!("goodbye");
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

#[macro_export]
macro_rules! assert_once {
    () => {
        static CALLED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
        CALLED
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            )
            .expect("this function may only be called once");
    };
}
