use alloc::boxed::Box;
use x86_64::{
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

pub fn init() {
    let mut idt = Box::new(InterruptDescriptorTable::new());

    set_general_handler!(&mut idt, general_handler);

    Box::leak(idt).load();
}

#[allow(clippy::needless_pass_by_value)]
fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    todo!("interrupt {index} ({error_code:?})\n{stack_frame:#?}");
}
