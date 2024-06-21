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

use boot::SMP_RESPONSE;
use cpu_data::CpuData;
use limine::smp::Cpu;
use x86_64::instructions::{hlt, interrupts};

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    logger::init();
    log::info!("Hello, World!");
    boot::verify();

    SMP_RESPONSE
        .cpus()
        .iter()
        .filter(|cpu| cpu.lapic_id != SMP_RESPONSE.bsp_lapic_id())
        .for_each(|cpu| {
            cpu.goto_address.write(main);
        });

    main(
        SMP_RESPONSE
            .cpus()
            .iter()
            .find(|cpu| cpu.lapic_id == SMP_RESPONSE.bsp_lapic_id())
            .expect("There should be a bsp"),
    );
}

extern "C" fn main(cpu: &Cpu) -> ! {
    log::info!("CPU {} is started", cpu.lapic_id);

    // Safety: This is the only place where CpuData is initialized
    unsafe { CpuData::init(u64::from(cpu.lapic_id)) };

    gdt::init();
    idt::init();

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
