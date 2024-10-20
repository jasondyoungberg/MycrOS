use core::ptr;

use alloc::boxed::Box;
use gdt::GlobalDescriptorTable;
use idt::IDT;
use tss::TaskStateSegment;

pub mod gdt;
pub mod idt;
pub mod tss;

#[repr(C, packed(2))]
struct DescriptorTablePointer<T> {
    limit: u16,
    base: *const T,
}

impl<T> DescriptorTablePointer<T> {
    pub fn new(table: &T) -> Self {
        DescriptorTablePointer {
            limit: (size_of::<T>().checked_sub(1).expect("Table is zero sized"))
                .try_into()
                .expect("Table is too big (> 65536 bytes)"),
            base: ptr::from_ref(table),
        }
    }
}

pub fn init() {
    let tss = Box::leak(Box::new(TaskStateSegment::new()));
    let gdt = Box::leak(Box::new(GlobalDescriptorTable::new(tss)));
    gdt.load();

    IDT.load();
}
