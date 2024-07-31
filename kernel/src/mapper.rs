use spin::{Lazy, Mutex};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        mapper::{MapToError, UnmapError},
        Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
    },
    VirtAddr,
};

use crate::{
    alloc_frame::{alloc_frame, dealloc_frame, FrameAllocator},
    boot::HHDM_RESPONSE,
};

static OFFSET: Lazy<VirtAddr> = Lazy::new(|| VirtAddr::new(HHDM_RESPONSE.offset()));

static KERNEL_MAPPER: Lazy<Mutex<OffsetPageTable>> = Lazy::new(|| {
    let frame = Cr3::read().0;
    let virt = *OFFSET + frame.start_address().as_u64();
    let ptr = virt.as_mut_ptr::<PageTable>();
    // Safety: this is only called once, so we know that ptr isn't aliased
    let table = unsafe { &mut *ptr };
    // Safety: we know that the table is valid because it came from Cr3.
    let mapper = unsafe { OffsetPageTable::new(table, *OFFSET) };
    Mutex::new(mapper)
});

/// # Safety
/// Memory mapping is fundamentally unsafe. See [`x86_64::structures::paging::Mapper::map_to`].
pub unsafe fn map_kernel_page_to_frame(
    page: Page,
    frame: PhysFrame,
    flags: PageTableFlags,
) -> Result<(), MapToError<Size4KiB>> {
    let mut mapper = KERNEL_MAPPER.lock();
    // Safety: handled by the caller
    unsafe { mapper.map_to(page, frame, flags, &mut FrameAllocator) }?.flush();

    Ok(())
}

/// # Safety
/// Memory mapping is fundamentally unsafe. See [`x86_64::structures::paging::Mapper::map_to`].
pub unsafe fn map_kernel_page(
    page: Page,
    flags: PageTableFlags,
) -> Result<(), MapToError<Size4KiB>> {
    let frame = alloc_frame().expect("Out of memory");
    // Safety: handled by the caller
    unsafe { map_kernel_page_to_frame(page, frame, flags) }
}

/// # Safety
/// Memory mapping is fundamentally unsafe.
pub unsafe fn unmap_kernel_page(page: Page) -> Result<(), UnmapError> {
    let mut mapper = KERNEL_MAPPER.lock();

    let (frame, flush) = mapper.unmap(page)?;
    flush.flush();

    // Safety: Frame was just deallocated and is therefore unused
    unsafe { dealloc_frame(frame) };

    Ok(())
}

pub fn create_l4_table() -> PhysFrame {
    let frame = alloc_frame().expect("Out of memory");
    let virt = *OFFSET + frame.start_address().as_u64();

    let ptr = virt.as_mut_ptr::<PageTable>();

    // Safety: We just allocated this frame, so it's safe to write to it
    unsafe { &mut *ptr }.clone_from(KERNEL_MAPPER.lock().level_4_table());

    frame
}
