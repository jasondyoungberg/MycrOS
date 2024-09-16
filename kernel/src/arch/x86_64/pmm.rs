use core::ptr::NonNull;

use limine::{memory_map::EntryType, request::MemoryMapRequest};
use spin::Mutex;

use crate::arch::{PhysPtr, PAGE_SIZE};

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

static HEAD: Mutex<Head> = Mutex::new(Head { next: None });

struct Head {
    next: Option<NonNull<Node>>,
}

unsafe impl Send for Head {}

struct Node {
    next: Option<NonNull<Node>>,
    count: usize,
    phys: PhysPtr<()>,
}

unsafe impl Send for Node {}

pub(super) fn init() {
    crate::assert_once!();

    let response = MEMORY_MAP_REQUEST.get_response().unwrap();

    let mut head = HEAD.lock();

    for entry in response.entries() {
        if entry.entry_type == EntryType::USABLE {
            let base = PhysPtr::new(entry.base as usize);
            let ptr = base.cast::<Node>().as_nonnull();

            unsafe {
                ptr.write(Node {
                    next: head.next,
                    count: entry.length as usize / PAGE_SIZE,
                    phys: base,
                })
            };

            head.next = Some(ptr);
        }
    }
}

pub fn alloc_page() -> Option<PhysPtr<()>> {
    let mut head = HEAD.lock();

    let old_first_ptr = head.next?;
    let old_first = unsafe { old_first_ptr.read() };

    Some(match old_first.count {
        2.. => {
            let new_first_ptr = unsafe { old_first_ptr.byte_add(PAGE_SIZE) };
            unsafe {
                new_first_ptr.write(Node {
                    next: old_first.next,
                    count: old_first.count - 1,
                    phys: old_first.phys.byte_add(PAGE_SIZE),
                })
            };
            head.next = Some(new_first_ptr);
            old_first.phys
        }
        1 => {
            head.next = old_first.next;
            old_first.phys
        }
        0 => panic!(),
    })
}

pub fn alloc_page_zerod() -> Option<PhysPtr<()>> {
    let page = alloc_page()?;

    unsafe {
        page.cast::<[u8; PAGE_SIZE]>()
            .as_mut_ptr()
            .write([0; PAGE_SIZE])
    };

    Some(page)
}
