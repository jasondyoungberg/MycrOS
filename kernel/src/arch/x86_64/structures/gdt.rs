use core::arch::asm;

use bit_field::BitField;

use crate::arch::x86_64::structures::DescriptorTablePointer;

use super::tss::TaskStateSegment;

pub const KERNEL_CODE: u16 = 8;
pub const KERNEL_DATA: u16 = 16;
pub const USER_DATA: u16 = 24 + 3;
pub const USER_CODE: u16 = 32 + 3;

#[repr(C, packed(8))]
pub struct GlobalDescriptorTable {
    null: u64,
    kernel_code: u64,
    kernel_data: u64,
    user_data: u64,
    user_code: u64,
    tss: u128,
}

impl GlobalDescriptorTable {
    pub fn new(tss: &'static TaskStateSegment) -> Self {
        unsafe { Self::new_unsafe(tss) }
    }

    pub unsafe fn new_unsafe(tss: &TaskStateSegment) -> Self {
        let tss_ptr = DescriptorTablePointer::new(tss);
        let base = tss_ptr.base as u128;
        let limit = tss_ptr.limit as u128;

        let mut descriptor = 0u128;
        descriptor.set_bits(0..16, limit);
        descriptor.set_bits(16..40, base.get_bits(0..24));
        descriptor.set_bit(47, true); // present
        descriptor.set_bits(56..96, base.get_bits(24..64));

        GlobalDescriptorTable {
            null: 0,
            kernel_code: 0x00af9b000000ffff,
            kernel_data: 0x00af93000000ffff,
            user_data: 0x00aff3000000ffff,
            user_code: 0x00affb000000ffff,
            tss: descriptor,
        }
    }

    pub fn load(&'static self) {
        unsafe { self.load_unsafe() }
    }

    pub unsafe fn load_unsafe(&self) {
        let gdtr = DescriptorTablePointer::new(self);

        asm!("
            lgdt [{gdtr}]

            push {code}
            lea {tmp}, [2f + rip]
            push {tmp}
            retfq
            2:

            mov ds, {data:x}
            mov es, {data:x}
            mov fs, {data:x}
            mov gs, {data:x}
            mov ss, {data:x}
            ",
            gdtr = in(reg) &gdtr,
            tmp = out(reg) _,
            code = in(reg) KERNEL_CODE as u64,
            data = in(reg) KERNEL_DATA,
        )
    }
}
