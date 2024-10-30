#[cfg(not(target_arch = "x86_64"))]
compile_error!("this code only works on x86_64");

use core::arch::asm;

use registers::control::Cr3;

use crate::{
    assert_once_percpu, boot,
    mem::{Mapper, KERNEL_MAPPER},
    println,
};

mod percpu;
mod registers;
mod structures;

/// # Safety
/// This function must be called exactly once per core
/// `cpuid` must be a unique int within 0..cpu_count
pub fn init(cpuid: u32) {
    let cpus = boot::cpu_count();
    assert!(cpuid < cpus);
    assert_once_percpu!(cpuid);

    unsafe { Cr3::write_raw(KERNEL_MAPPER.lock().ptroot().addr() as u64) };

    println!("initilizing gdt/tss...");
    structures::init();

    unsafe { percpu::init(cpuid) };
}

pub fn get_cpuid() -> u32 {
    percpu::get_percpu().cpuid
}

pub fn debug_print(s: &str) {
    unsafe {
        asm!(
            "rep outs dx, byte ptr [rsi]",
            in("dx") 0xe9,
            in("rsi") s.as_ptr(),
            in("rcx") s.len(),
        )
    }
}

pub fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
