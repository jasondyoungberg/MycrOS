use core::{cell::Cell, ptr};

use alloc::{boxed::Box, sync::Arc};
use spin::Mutex;
use x86_64::{registers::model_specific::GsBase, structures::tss::TaskStateSegment, VirtAddr};

use crate::process::Process;

#[derive(Debug)]
#[repr(C)]
// NOTE: Don't move self_ptr, syscall_rsp, or sysret_rsp, as they are accessed by assembly code
pub struct CpuData {
    self_ptr: *const CpuData,
    syscall_rsp: Cell<VirtAddr>,
    sysret_rsp: Cell<VirtAddr>,
    magic: [u8; 8],
    pub cpuid: u64,
    pub tss: Mutex<TaskStateSegment>,
    pub active_process: Mutex<Option<Arc<Mutex<Process>>>>,
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
            tss: Mutex::new(TaskStateSegment::new()),
            active_process: Mutex::new(None),
        }));

        // Safety: This is the only reference to data
        unsafe { &mut *data }.self_ptr = data;

        let vaddr = VirtAddr::from_ptr(data);

        GsBase::write(vaddr);
    }

    pub fn get() -> &'static Self {
        // let ptr: *const Self;
        // Safety: we a reading the `CpuData.self_ptr`, which shouldn't have any side effects
        // unsafe { asm!("mov {}, gs:0", out(reg) ptr) }

        let ptr = GsBase::read().as_ptr::<Self>();

        assert!((ptr as i64) < 0, "Invalid pointer {ptr:p}");

        // Safety: Only immutable references exist at this point
        let data = unsafe { &*ptr };

        assert_eq!(data.self_ptr, ptr, "Pointers don't match");
        assert_eq!(data.magic, *MAGIC, "Magic doesn't match");

        data
    }
}
