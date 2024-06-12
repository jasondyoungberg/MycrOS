#![no_std]
#![no_main]

mod alloc_frame;
mod boot;
mod logger;

use x86_64::instructions::{hlt, interrupts};

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    boot::verify();
    logger::init();

    log::info!("Hello, World!");

    log::error!("error");
    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
    log::trace!("trace");

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    interrupts::disable();
    loop {
        hlt();
    }
}
