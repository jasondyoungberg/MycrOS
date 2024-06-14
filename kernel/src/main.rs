#![no_std]
#![no_main]

extern crate alloc;

mod alloc_frame;
mod boot;
mod display;
mod gdt;
mod heap;
mod layout;
mod logger;
mod mapper;

use x86_64::instructions::{hlt, interrupts};

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    logger::init();
    log::info!("Hello, World!");
    boot::verify();
    mapper::init();

    gdt::init();

    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{}", info);
    hcf();
}

fn hcf() -> ! {
    interrupts::disable();
    loop {
        hlt();
    }
}
