use core::{cell::UnsafeCell, mem::MaybeUninit};

use super::PAGE_SIZE;

#[repr(C, align(4096))]
pub struct Page(UnsafeCell<MaybeUninit<[u8; PAGE_SIZE]>>);

impl Page {
    pub fn fill_zero(&mut self) {
        self.0.get_mut().write([0; PAGE_SIZE]);
    }
}

unsafe impl Send for Page {}
unsafe impl Sync for Page {}
