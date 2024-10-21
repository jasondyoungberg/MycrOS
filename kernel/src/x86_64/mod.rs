#[cfg(not(target_arch = "x86_64"))]
compile_error!("this code only works on x86_64");

use registers::control::Cr3;

use crate::{
    mem::{Mapper, KERNEL_MAPPER},
    println,
};

pub mod registers;
mod structures;

pub fn init() {
    unsafe { Cr3::write_raw(KERNEL_MAPPER.lock().ptroot().addr() as u64) };

    println!("initilizing gdt/tss...");
    structures::init();
}
