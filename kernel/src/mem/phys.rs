use core::alloc::AllocError;
use core::ptr::NonNull;

use spin::{Lazy, Mutex};

use crate::{boot::phys_memmap_usable, println};

use super::{Page, PhysPtr, PAGE_SIZE};

static HEAD: Lazy<Mutex<Head>> = Lazy::new(|| {
    println!("init pmm");

    let mut head = Head { next: None };

    for (phys, size) in phys_memmap_usable() {
        assert!(size % PAGE_SIZE == 0, "size is not page aligned");

        for i in 0..(size / PAGE_SIZE) {
            unsafe { dealloc_inner(&mut head, phys.cast::<Page>().add(i)) };
        }
    }

    let res = Mutex::new(head);

    println!("pmm ready");
    res
});

struct Head {
    next: Option<NonNull<Node>>,
}

unsafe impl Send for Head {}

struct Node {
    next: Option<NonNull<Node>>,
    phys: PhysPtr<Page>,
}

unsafe impl Send for Node {}

/// # Invariants
/// - result page is unused
pub fn alloc() -> Result<PhysPtr<Page>, AllocError> {
    let mut head = HEAD.lock();

    let first_ptr = head.next.ok_or(AllocError)?;
    let first = unsafe { first_ptr.read() };

    Ok({
        head.next = first.next;
        first.phys
    })
}

/// # Invariants
/// - result page is unused
/// - result page only contains zeros
pub fn alloc_zeroed() -> Result<PhysPtr<Page>, AllocError> {
    let page = alloc()?;
    let mut ptr = page.as_nonnull();

    unsafe { ptr.as_mut() }.fill_zero();

    Ok(page)
}

/// # Safety
/// - page must be unused
pub unsafe fn dealloc(page: PhysPtr<Page>) {
    dealloc_inner(&mut HEAD.lock(), page);
}

unsafe fn dealloc_inner(head: &mut Head, page: PhysPtr<Page>) {
    let ptr = page.cast::<Node>().as_nonnull();

    unsafe {
        ptr.write(Node {
            next: head.next,
            phys: page,
        })
    };

    head.next = Some(ptr);
}
