mod mapping;
mod paging;
pub mod phys;
mod physptr;

pub use mapping::*;
pub use paging::*;
pub use physptr::*;

pub const PAGE_SIZE: usize = 4096;
pub const MAX_PHYS_ADDR: usize = 1024 * 1024 * 1024 * 1024; // 1 TiB
pub const HIGHER_HALF_ADDR: usize = 0x8000_0000_0000_0000;

pub fn init() {}
