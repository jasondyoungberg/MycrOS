use core::{
    arch::global_asm,
    cell::Cell,
    ops::Deref,
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};

use alloc::boxed::Box;
use spin::Once;

use crate::{
    assert_once_percpu,
    boot::{self, cpu_count},
    x86_64::registers::model_specific::GsBase,
};

const MAGIC: u64 = u64::from_le_bytes(*b"!percpu!");

global_asm!("
.section .text
.global foo
foo:
    mov rax, {VAL}
    ret
", VAL = const 69);

#[repr(C)]
pub struct PerCpu {
    selfptr: *const PerCpu,
    magic: u64,
    syscall_rsp: usize,
    sysret_rsp: usize,
    cpuid: u32,
}

static WAITING: AtomicU32 = AtomicU32::new(u32::MAX);

/// # Safety
/// This function must be called exactly once per core
/// `cpuid` must be a unique int within 0..cpu_count
pub unsafe fn init(cpuid: u32) {
    let cpus = boot::cpu_count();
    assert!(cpuid < cpus);
    assert_once_percpu!(cpuid);

    let _ = WAITING.compare_exchange(u32::MAX, cpus, Ordering::AcqRel, Ordering::Acquire);

    let percpu = Box::leak(Box::new(PerCpu {
        selfptr: ptr::null(),
        magic: MAGIC,
        syscall_rsp: 0,
        sysret_rsp: 0,
        cpuid,
    }));
    let percpu_ptr = percpu as *const PerCpu;
    percpu.selfptr = percpu_ptr;

    unsafe { GsBase::write_raw(percpu_ptr as usize as u64) };
    WAITING.fetch_sub(1, Ordering::AcqRel);
}

pub fn get_percpu() -> &'static PerCpu {
    loop {
        let waiting = WAITING.load(Ordering::Acquire);
        match waiting {
            0 => break,
            u32::MAX => panic!("attempted to get percpu without initilizing"),
            _ => continue,
        }
    }

    let percpu_ptr = unsafe { GsBase::read_raw() } as usize as *const PerCpu;
    assert_eq!(unsafe { (*percpu_ptr).selfptr }, percpu_ptr);
    assert_eq!(unsafe { (*percpu_ptr).magic }, MAGIC);
    unsafe { &*percpu_ptr }
}

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
        })[get_percpu().cpuid as usize]
    }
}

impl<T> Deref for CpuLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.force()
    }
}
