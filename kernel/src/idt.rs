use core::pin::Pin;

use alloc::boxed::Box;
use spin::{Lazy, Mutex};
use x86_64::{
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

pub static IDT: Lazy<Mutex<Pin<Box<InterruptDescriptorTable>>>> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    Mutex::new(Box::pin(idt))
});

pub fn init() {
    let idt = IDT.lock();
    // Safety: This is safe since the IDT is in a static pinned box.
    unsafe { idt.load_unsafe() };
}

#[allow(clippy::needless_pass_by_value)]
fn general_handler(_stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    if let Some(error_code) = error_code {
        log::warn!("interrupt {index} ({error_code})");
    } else {
        log::warn!("interrupt {index}");
    }
}
