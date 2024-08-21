use core::marker::PhantomData;

use crate::boot::HHDM_OFFSET;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PhysPtr<T> {
    addr: usize,
    _phantom: PhantomData<T>,
}

impl<T> PhysPtr<T> {
    pub const fn new(addr: usize) -> Self {
        PhysPtr {
            addr,
            _phantom: PhantomData,
        }
    }

    pub const fn cast<U>(&self) -> PhysPtr<U> {
        PhysPtr::new(self.addr)
    }

    pub fn as_ptr(&self) -> *const T {
        (self.addr + *HHDM_OFFSET) as *const T
    }
}
