use core::pin::Pin;

use alloc::boxed::Box;
use spin::{Lazy, Mutex};
use x86_64::{
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, InterruptStackFrameValue},
};

use crate::exception;

pub static IDT: Lazy<Mutex<Pin<Box<InterruptDescriptorTable>>>> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    let o = idt.page_fault.set_handler_fn(exception::page_fault);
    unsafe { o.set_stack_index(1) };

    let o = idt.double_fault.set_handler_fn(exception::double_fault);
    unsafe { o.set_stack_index(2) };

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

#[derive(Debug)]
#[repr(C)]
pub struct Context {
    pub registers: [u64; 15],
    pub code: u64,
    pub frame: InterruptStackFrameValue,
}

#[macro_export]
macro_rules! wrap {
    (swapgs) => {
        "
        cmp word ptr [rsp + 16], 8
        je 2f
        swapgs
        2:
        "
    };
    (push_all) => {
        "
        push r15; push r14; push r13; push r12;
        push r11; push r10; push r9;  push r8;
                  push rbp; push rdi; push rsi;
        push rdx; push rcx; push rbx; push rax;
        "
    };
    (pop_all) => {
        "
        pop rax; pop rbx; pop rcx; pop rdx;
        pop rsi; pop rdi; pop rbp;
        pop r8;  pop r9;  pop r10; pop r11;
        pop r12; pop r13; pop r14; pop r15;
        "
    };
    (irq, $inner:ident => $wrapper:ident) => {
        const _: unsafe extern "C" fn(&mut $crate::idt::Context) = $inner;

        #[naked]
        pub extern "x86-interrupt" fn $wrapper(
            _stack_frame: x86_64::structures::idt::InterruptStackFrame
        ) {
            // Safety: This should be safe
            unsafe {
                core::arch::asm!(
                    "push 0",
                    $crate::wrap!(swapgs),
                    $crate::wrap!(push_all),
                    "mov rdi, rsp"
                    "call {inner}"
                    $crate::wrap!(pop_all),
                    $crate::wrap!(swapgs),
                    "add rsp, 8",
                    "iretq",

                    inner = sym $inner,
                    options(noreturn)
                )
            }
        }
    };
    (irq($code:ty), $inner:ident => $wrapper:ident) => {
        $crate::wrap!(irq($code) -> (), $inner => $wrapper);
    };
    (irq($code:ty) -> $ret:ty, $inner:ident => $wrapper:ident) => {
        const _: unsafe extern "C" fn(&mut $crate::idt::Context) -> $ret = $inner;

        #[naked]
        pub extern "x86-interrupt" fn $wrapper(
            _stack_frame: x86_64::structures::idt::InterruptStackFrame,
            _code: $code
        ) -> $ret {
            // Safety: This should be safe
            unsafe {
                core::arch::asm!(
                    $crate::wrap!(swapgs),
                    $crate::wrap!(push_all),
                    "mov rdi, rsp",
                    "call {inner}",
                    $crate::wrap!(pop_all),
                    $crate::wrap!(swapgs),
                    "add rsp, 8",
                    "iretq",

                    inner = sym $inner,
                    options(noreturn)
                )
            }
        }
    };
}
