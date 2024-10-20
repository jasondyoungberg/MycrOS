use core::{cell::Cell, ops::Deref};

use alloc::boxed::Box;
use spin::Once;

use crate::boot::cpu_count;

pub struct CpuLocal<T> {
    cell: Once<Box<[T]>>,
    init: Cell<Option<fn(usize) -> T>>,
}

impl<T> CpuLocal<T> {
    pub const fn new(f: fn(usize) -> T) -> Self {
        Self {
            cell: Once::new(),
            init: Cell::new(Some(f)),
        }
    }

    pub fn force(&self) -> &T {
        self.cell.call_once(|| match self.init.take() {
            Some(f) => (0..cpu_count()).map(f).collect(),
            None => panic!("CpuLocal instance has previously been poisoned"),
        });
        todo!()
    }
}

impl<T> Deref for CpuLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
