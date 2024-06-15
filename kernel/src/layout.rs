use x86_64::{
    structures::paging::{page::PageRange, Page},
    VirtAddr,
};

pub const HEAP: PageRange = Page::range(
    Page::containing_address(VirtAddr::new(0xFFFF_A000_0000_0000)),
    Page::containing_address(VirtAddr::new(0xFFFF_A100_0000_0000)),
);

pub const STACK: PageRange = Page::range(
    Page::containing_address(VirtAddr::new(0xFFFF_A100_0000_0000)),
    Page::containing_address(VirtAddr::new(0xFFFF_A200_0000_0000)),
);
