#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

extern crate alloc;

mod alloc_frame;
mod alloc_page;
mod boot;
mod cpu_data;
mod display;
mod exception;
mod gdt;
mod heap;
mod idt;
mod layout;
mod logger;
mod mapper;
mod stack;

use cpu_data::CpuData;
use x86_64::instructions::{hlt, interrupts};

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    logger::init();
    log::info!("Hello, World!");
    boot::verify();
    mapper::init();

    gdt::init();
    idt::init();

    // Safety: This is the only place where CpuData is initialized
    unsafe { CpuData::init(0) };

    interrupts::enable();

    loop {
        hlt();
    }
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
