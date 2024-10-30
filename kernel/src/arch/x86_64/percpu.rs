use core::{
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};

use alloc::boxed::Box;

use crate::boot;

use super::registers::model_specific::GsBase;

#[repr(C)]
pub struct PerCpu {
    selfptr: *const PerCpu,
    magic: u64,
    syscall_rsp: usize,
    sysret_rsp: usize,
    pub cpuid: u32,
}

const MAGIC: u64 = u64::from_le_bytes(*b"!percpu!");

static WAITING: AtomicU32 = AtomicU32::new(u32::MAX);

pub unsafe fn init(cpuid: u32) {
    let _ = WAITING.compare_exchange(
        u32::MAX,
        boot::cpu_count(),
        Ordering::AcqRel,
        Ordering::Acquire,
    );

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
            _ => core::hint::spin_loop(),
        }
    }

    let percpu_ptr = unsafe { GsBase::read_raw() } as usize as *const PerCpu;
    assert_eq!(unsafe { (*percpu_ptr).selfptr }, percpu_ptr);
    assert_eq!(unsafe { (*percpu_ptr).magic }, MAGIC);
    unsafe { &*percpu_ptr }
}
