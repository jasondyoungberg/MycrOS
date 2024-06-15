use core::sync::atomic::{AtomicU64, Ordering};

use x86_64::{
    structures::paging::{page::PageRange, Page},
    VirtAddr,
};

pub struct PageAllocator {
    range: PageRange,
    next: AtomicU64,
}

impl PageAllocator {
    pub const fn new(range: PageRange) -> Self {
        Self {
            range,
            next: AtomicU64::new(range.start.start_address().as_u64()),
        }
    }

    pub fn alloc_pages(&self, count: u64) -> Option<PageRange> {
        let size = count * 4096;
        let start = self.next.fetch_add(size, Ordering::Relaxed);
        let start = VirtAddr::new(start);
        let end = start + size;

        if start >= self.range.end.start_address() {
            None
        } else {
            Some(Page::range(
                Page::from_start_address(start).expect("Should be page aligned"),
                Page::from_start_address(end).expect("Should be page aligned"),
            ))
        }
    }
}
