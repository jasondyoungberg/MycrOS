use core::sync::atomic::{AtomicUsize, Ordering};

use crate::mem::{Mapper, MappingError, MappingKind, KERNEL_MAPPER, PAGE_SIZE};

pub struct Stack {
    vaddr: usize,
}

pub const STACK_SIZE: usize = 256 * 1024;

static STACK_VADDR: AtomicUsize = AtomicUsize::new(0xFFFF_A000_0000_0000);

impl Stack {
    pub fn new() -> Result<Self, MappingError> {
        let vaddr = STACK_VADDR.fetch_add(STACK_SIZE + 2 * PAGE_SIZE, Ordering::Relaxed);
        let ptr = vaddr as *mut ();

        let mut mapper = KERNEL_MAPPER.lock();
        unsafe { mapper.map(ptr, PAGE_SIZE, MappingKind::Gaurd) }?;
        unsafe {
            mapper.map(
                ptr.wrapping_byte_add(PAGE_SIZE),
                STACK_SIZE,
                MappingKind::ReadWrite,
            )
        }?;
        unsafe {
            mapper.map(
                ptr.wrapping_byte_add(PAGE_SIZE + STACK_SIZE),
                PAGE_SIZE,
                MappingKind::Gaurd,
            )
        }?;
        drop(mapper);

        Ok(Self { vaddr })
    }

    pub fn stack_pointer(&self) -> usize {
        self.vaddr + STACK_SIZE + PAGE_SIZE
    }
}
