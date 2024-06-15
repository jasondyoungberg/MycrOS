use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

pub extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    code: PageFaultErrorCode,
) {
    let cr2 = Cr2::read();
    panic!("Page Fault\nCode: {code:?}\nAddress: {cr2:?}\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, _code: u64) -> ! {
    panic!("Double Fault\n{stack_frame:#?}");
}
