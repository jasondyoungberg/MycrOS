use x86_64::{structures::paging::page::PageRange, VirtAddr};

use crate::{alloc_page::PageAllocator, layout, mapper::unmap_kernel_page};

static PAGE_ALLOC: PageAllocator = PageAllocator::new(layout::STACK);

pub struct Stack {
    pages: PageRange,
}

impl Stack {
    pub fn new(size: u64) -> Self {
        assert_eq!(size % 4096, 0, "size should be a multiple of 4096");

        let pages = PAGE_ALLOC
            .alloc_pages(size / 4096)
            .expect("Stack address range depleted");

        Self { pages }
    }

    /// # Safety
    /// The stack must be unused
    pub unsafe fn drop(&self) {
        for page in self.pages {
            // Safety: this stack is unused
            unsafe { unmap_kernel_page(page) }.expect("Unmapping failed");
        }
    }

    pub fn rsp(&self) -> VirtAddr {
        (self.pages.end - 1).start_address()
    }
}
