use spin::Mutex;
use talc::{Span, Talc, Talck};

use crate::mem::{Mapper, MappingKind, KERNEL_MAPPER, PAGE_SIZE};

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, MyOomHandler> = Talc::new(MyOomHandler).lock();

const HEAP_START: *mut u8 = 0xffff_9000_0000_0000 as *mut u8;
static HEAP_SPAN: Mutex<Span> = Mutex::new(Span::empty());

struct MyOomHandler;

impl talc::OomHandler for MyOomHandler {
    fn handle_oom(talc: &mut Talc<Self>, _layout: core::alloc::Layout) -> Result<(), ()> {
        let mut heap_span = HEAP_SPAN.try_lock().expect("lock should always work");

        if let Some((_base, acme)) = heap_span.get_base_acme() {
            // add one more page
            unsafe {
                KERNEL_MAPPER
                    .lock()
                    .map(acme.cast(), PAGE_SIZE, MappingKind::ReadWrite)
            }
            .map_err(|_| ())?;
            let old_span = *heap_span;
            let new_span = heap_span.extend(0, PAGE_SIZE);
            *heap_span = unsafe { talc.extend(old_span, new_span) };
            Ok(())
        } else {
            // init heap
            unsafe {
                KERNEL_MAPPER
                    .lock()
                    .map(HEAP_START.cast(), PAGE_SIZE, MappingKind::ReadWrite)
            }
            .map_err(|_| ())?;
            *heap_span = unsafe { talc.claim(Span::from_base_size(HEAP_START, PAGE_SIZE)) }?;
            Ok(())
        }
    }
}
