use x86_64::{registers::control::Cr2, structures::idt::PageFaultErrorCode};

use crate::{idt::Context, wrap};

wrap!(irq(PageFaultErrorCode), page_fault_inner => page_fault);
pub extern "C" fn page_fault_inner(context: &mut Context) {
    let code = PageFaultErrorCode::from_bits_retain(context.code);
    let cr2 = Cr2::read();
    let frame = context.frame;
    panic!("Page Fault\nCode: {code:?}\nAddress: {cr2:?}\n{frame:#?}");
}

wrap!(irq(u64) -> !, double_fault_inner => double_fault);
pub extern "C" fn double_fault_inner(context: &mut Context) -> ! {
    let frame = context.frame;
    panic!("Double Fault\n{frame:#?}");
}
