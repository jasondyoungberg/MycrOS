use core::alloc::GlobalAlloc;

use spin::Mutex;
use x86_64::{
    structures::paging::{page::PageRange, Page, PageTableFlags},
    VirtAddr,
};

use crate::{layout, mapper::map_kernel_page};

#[global_allocator]
static HEAP: Heap = unsafe { Heap::new(layout::HEAP) };

struct Heap {
    data: Mutex<HeapData>,
}
impl Heap {
    /// # Safety
    /// The range must be unused for any other purpose and must always be mapped.
    const unsafe fn new(range: PageRange) -> Self {
        // Safety: Handled by the caller.
        let data = unsafe { HeapData::new(range) };
        Self {
            data: Mutex::new(data),
        }
    }
}

// Safety: The provided pointers are always unique valid, with the correct alignment and size.
unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut data = self.data.lock();

        let res_start = data.next.align_up(layout.align() as u64);
        let res_end = res_start + layout.size() as u64;

        data.next = res_end;

        while data.brk < data.next {
            let page =
                Page::from_start_address(data.brk).expect("`data.end` should be page aligned");
            let flags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT;

            // Safety: The page is unused for any other purpose
            unsafe { map_kernel_page(page, flags) }.expect("Mapping failed");
            data.brk += 4096;

            assert!(data.brk < data.end, "Heap address range depleted");
        }

        res_start.as_mut_ptr()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // Do nothing
    }
}

struct HeapData {
    next: VirtAddr,
    brk: VirtAddr,
    end: VirtAddr,
}
impl HeapData {
    const unsafe fn new(range: PageRange) -> Self {
        Self {
            next: range.start.start_address(),
            brk: range.start.start_address(),
            end: range.end.start_address(),
        }
    }
}
