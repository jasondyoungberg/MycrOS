pub mod debug;
pub mod pmm;
mod structures;
pub mod vmm;

pub const PAGE_SIZE: usize = 4096;

pub fn init() {
    crate::assert_once!();

    structures::init();
    pmm::init();
}
