use spin::Mutex;
use talc::{Span, Talc, Talck};

use crate::arch::{memory::MappingKind, pmm::alloc_page, vmm::create_mapping, PAGE_SIZE};

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, MyOomHandler> = Talc::new(MyOomHandler).lock();

const HEAP_START: *mut u8 = 0xffff_9000_0000_0000 as *mut u8;
static HEAP_SPAN: Mutex<Span> = Mutex::new(Span::empty());

struct MyOomHandler;

impl talc::OomHandler for MyOomHandler {
    fn handle_oom(talc: &mut Talc<Self>, _layout: core::alloc::Layout) -> Result<(), ()> {
        let page = alloc_page().unwrap();

        let mut heap_span = HEAP_SPAN.try_lock().unwrap();

        if let Some((_base, acme)) = heap_span.get_base_acme() {
            unsafe { create_mapping(acme.cast(), page, PAGE_SIZE, MappingKind::ReadWrite) };
            let old_span = *heap_span;
            let new_span = heap_span.extend(0, PAGE_SIZE);
            *heap_span = unsafe { talc.extend(old_span, new_span) };
            Ok(())
        } else {
            unsafe { create_mapping(HEAP_START.cast(), page, PAGE_SIZE, MappingKind::ReadWrite) };
            *heap_span = unsafe { talc.claim(Span::from_base_size(HEAP_START, PAGE_SIZE)) }?;
            Ok(())
        }
    }
}
