use crate::println;

pub mod debug;
pub mod pmm;
mod structures;
pub mod vmm;

pub const PAGE_SIZE: usize = 4096;

pub fn init() {
    crate::assert_once!();

    println!("initilizing gdt/tss...");
    structures::init();
    println!("initilizing pmm...");
    pmm::init();
    println!("initilizing vmm...");
    vmm::init();
}
