use alloc::boxed::Box;
use x86_64::{
    registers::segmentation::{Segment, CS, DS, ES, FS, GS, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    PrivilegeLevel::{Ring0, Ring3},
};

use crate::stack::Stack;

const KERNEL_CODE: SegmentSelector = SegmentSelector::new(1, Ring0);
const KERNEL_DATA: SegmentSelector = SegmentSelector::new(2, Ring0);
const USER_DATA: SegmentSelector = SegmentSelector::new(3, Ring3);
const USER_CODE: SegmentSelector = SegmentSelector::new(4, Ring3);

pub fn init() {
    let mut tss = TaskStateSegment::new();

    tss.privilege_stack_table[0] = Stack::new(65536).rsp();

    for i in 0..7 {
        tss.interrupt_stack_table[i] = Stack::new(65536).rsp();
    }

    let mut gdt = Box::new(GlobalDescriptorTable::new());
    let kernel_code = gdt.append(Descriptor::kernel_code_segment());
    let kernel_data = gdt.append(Descriptor::kernel_data_segment());
    let user_data = gdt.append(Descriptor::user_data_segment());
    let user_code = gdt.append(Descriptor::user_code_segment());

    assert_eq!(
        kernel_code, KERNEL_CODE,
        "Kernel code selector not as expected"
    );
    assert_eq!(
        kernel_data, KERNEL_DATA,
        "Kernel data selector not as expected"
    );
    assert_eq!(user_data, USER_DATA, "User data selector not as expected");
    assert_eq!(user_code, USER_CODE, "User code selector not as expected");

    Box::leak(gdt).load();

    // Safety: These selectors are valid.
    unsafe {
        CS::set_reg(KERNEL_CODE);
        DS::set_reg(KERNEL_DATA);
        ES::set_reg(KERNEL_DATA);
        FS::set_reg(KERNEL_DATA);
        GS::set_reg(KERNEL_DATA);
        SS::set_reg(KERNEL_DATA);
    }
}
