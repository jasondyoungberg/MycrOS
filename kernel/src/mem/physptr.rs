use core::{fmt::Debug, marker::PhantomData, ptr::NonNull};

use crate::{boot::hhdm_offset, mem::MAX_PHYS_ADDR};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PhysPtr<T> {
    addr: usize,
    _phantom: PhantomData<*const T>,
}

unsafe impl<T> Send for PhysPtr<T> where T: Send {}

impl<T> Clone for PhysPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for PhysPtr<T> {}

impl<T> PhysPtr<T> {
    pub fn new(addr: usize) -> Self {
        assert!(addr % align_of::<T>() == 0, "addr is not correctly aligned");
        assert!(addr < MAX_PHYS_ADDR, "addr is too big");

        PhysPtr {
            addr,
            _phantom: PhantomData,
        }
    }

    pub fn addr(&self) -> usize {
        self.addr
    }

    pub fn cast<U>(&self) -> PhysPtr<U> {
        PhysPtr::new(self.addr)
    }

    pub fn as_ptr(&self) -> *const T {
        hhdm_offset().wrapping_byte_add(self.addr).cast()
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.as_ptr().cast_mut()
    }

    pub fn as_nonnull(&self) -> NonNull<T> {
        NonNull::new(self.as_mut_ptr()).expect("hhdm should never have null")
    }

    pub fn add(&self, count: usize) -> Self {
        Self::new(self.addr + size_of::<T>() * count)
    }

    pub fn byte_add(&self, count: usize) -> Self {
        Self::new(self.addr + count)
    }
}

impl<T> Debug for PhysPtr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PhysPtr({:#x})", self.addr)
    }
}
