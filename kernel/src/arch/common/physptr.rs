use core::{fmt::Debug, marker::PhantomData, ptr::NonNull};

use crate::boot::hhdm_offset;

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

    pub const fn addr(&self) -> usize {
        self.addr
    }

    pub const fn cast<U>(&self) -> PhysPtr<U> {
        PhysPtr::new(self.addr)
    }

    pub fn as_ptr(&self) -> *const T {
        hhdm_offset().byte_add(self.addr).addr as *const T
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.as_ptr().cast_mut()
    }

    pub fn as_nonnull(&self) -> NonNull<T> {
        NonNull::new(self.as_mut_ptr()).unwrap()
    }

    pub unsafe fn as_ref<'a>(&self) -> &'a T {
        self.as_ptr().as_ref().unwrap()
    }

    pub unsafe fn as_mut_ref<'a>(&self) -> &'a mut T {
        self.as_mut_ptr().as_mut().unwrap()
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
