use core::{arch::asm, cell::Cell, ptr};

use alloc::boxed::Box;
use x86_64::{registers::model_specific::GsBase, VirtAddr};

#[derive(Debug)]
#[repr(C)]
pub struct CpuData {
    self_ptr: *const CpuData,
    syscall_rsp: Cell<VirtAddr>,
    sysret_rsp: Cell<VirtAddr>,
    magic: [u8; 8],
    cpuid: u64,
}

const MAGIC: &[u8; 8] = b"CpuData!";

impl CpuData {
    /// # Safety
    /// This function must only be called once per cpu
    pub unsafe fn init(cpuid: u64) {
        let data = Box::into_raw(Box::new(Self {
            self_ptr: ptr::null(),
            syscall_rsp: Cell::new(VirtAddr::zero()),
            sysret_rsp: Cell::new(VirtAddr::zero()),
            magic: *MAGIC,
            cpuid,
        }));

        // Safety: This is the only reference to data
        unsafe { &mut *data }.self_ptr = data;

        GsBase::write(VirtAddr::from_ptr(data));
    }

    pub fn get() -> &'static Self {
        let ptr: *const Self;

        // Safety: we a reading the `CpuData.self_ptr`, which shouldn't have any side effects
        unsafe { asm!("mov {}, gs:0", out(reg) ptr) }

        // Safety: Only immutable references exist at this point
        let data = unsafe { &*ptr };

        assert_eq!(data.self_ptr, ptr, "Pointers don't match");
        assert_eq!(data.magic, *MAGIC, "Magic doesn't match");

        data
    }
}
