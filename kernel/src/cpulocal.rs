use core::{cell::Cell, ops::Deref};

use alloc::boxed::Box;
use spin::Once;

use crate::{arch, boot::cpu_count};

pub struct CpuLocal<T, F = fn(u32) -> T> {
    cell: Once<Box<[T]>>,
    init: Cell<Option<F>>,
}

unsafe impl<T, F: Send> Sync for CpuLocal<T, F> where Once<Box<[T]>>: Sync {}

impl<T> CpuLocal<T> {
    pub const fn new(f: fn(u32) -> T) -> Self {
        Self {
            cell: Once::new(),
            init: Cell::new(Some(f)),
        }
    }

    pub fn force(&self) -> &T {
        &self.cell.call_once(|| match self.init.take() {
            Some(f) => (0..cpu_count()).map(f).collect(),
            None => panic!("CpuLocal instance has previously been poisoned"),
        })[arch::get_cpuid() as usize]
    }
}

impl<T> Deref for CpuLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.force()
    }
}
