#[cfg(not(target_arch = "x86_64"))]
compile_error!("this code only works on x86_64");

use registers::Cr3;

use crate::{
    mem::{Mapper, KERNEL_MAPPER},
    println,
};

pub mod registers;
mod structures;

pub fn init() {
    let cr3 = Cr3(KERNEL_MAPPER.lock().ptroot().addr() as u64);
    unsafe { cr3.store() };

    println!("initilizing gdt/tss...");
    structures::init();
}
