use core::{
    arch::{asm, global_asm},
    array,
};

use bit_field::BitField;

use crate::{arch::x86_64::structures::DescriptorTablePointer, println};

use super::gdt::KERNEL_CODE;

global_asm!(include_str!("isr.asm"));

extern "C" {
    static isr_table: [usize; 256];
}

#[no_mangle]
extern "C" fn isr_inner(data: &IsrData) {
    const EXCEPTIONS: [&str; 32] = [
        "division error",
        "debug exception",
        "non maskable interrupt",
        "breakpoint",
        "overflow",
        "bound range exceeded",
        "invalid opcode",
        "device not available",
        "double fault",
        "coprocessor segment overrun",
        "invalid tss",
        "segment not present",
        "stack segment fault",
        "general protection fault",
        "page fault",
        "",
        "x87 floating point exception",
        "alignment check",
        "machine check",
        "simd floating point exception",
        "virtualiztion exception",
        "control protection exception",
        "",
        "",
        "",
        "",
        "",
        "",
        "hypervisor injection exception",
        "vmm communication exception",
        "security exception",
        "",
    ];

    let rip = data.instruction as usize;
    let vector = data.vector;
    match vector {
        3 => println!("int {vector}: {}\n\trip: {rip:#x}", EXCEPTIONS[vector]),
        0..32 => panic!("int {vector}: {}\n\trip: {rip:#x}", EXCEPTIONS[vector]),
        32..256 => println!("int {vector}"),
        256.. => unreachable!(),
    }
}

#[repr(transparent)]
pub struct InterruptDescriptorTable {
    data: [InterruptDescriptorTableEntry; 256],
}

#[repr(C)]
struct InterruptDescriptorTableEntry {
    isr_low: u16,
    kernel_code_segment: u16,
    ist: u8,
    attributes: u8,
    isr_mid: u16,
    isr_high: u32,
    _reserved: u32,
}

#[derive(Debug)]
#[repr(C)]
struct IsrData {
    registers: [usize; 16],
    vector: usize,
    err_code: usize,
    instruction: *const u8,
    code_segment: usize,
    flags: usize,
    stack_pointer: *const usize,
    stack_segment: usize,
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            data: array::from_fn(|i| {
                let isr = unsafe { isr_table[i] };
                InterruptDescriptorTableEntry {
                    isr_low: isr.get_bits(0..16) as u16,
                    kernel_code_segment: KERNEL_CODE,
                    ist: 0,
                    attributes: 0x8e,
                    isr_mid: isr.get_bits(16..32) as u16,
                    isr_high: isr.get_bits(32..64) as u32,
                    _reserved: 0,
                }
            }),
        }
    }

    pub fn load(&'static self) {
        unsafe { self.load_unsafe() }
    }

    pub unsafe fn load_unsafe(&self) {
        let idtr = DescriptorTablePointer::new(self);
        asm!("lidt [{}]", in(reg) &idtr);
    }
}
