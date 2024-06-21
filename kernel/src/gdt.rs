use alloc::boxed::Box;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, DS, ES, SS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    PrivilegeLevel::{Ring0, Ring3},
};

use crate::{cpu_data::CpuData, stack::Stack};

const KERNEL_CODE: SegmentSelector = SegmentSelector::new(1, Ring0);
const KERNEL_DATA: SegmentSelector = SegmentSelector::new(2, Ring0);
const USER_DATA: SegmentSelector = SegmentSelector::new(3, Ring3);
const USER_CODE: SegmentSelector = SegmentSelector::new(4, Ring3);

pub fn init() {
    let cpu_data = CpuData::get();
    let mut tss = cpu_data.tss.lock();

    tss.privilege_stack_table[0] = Stack::new(65536).rsp();

    for i in 0..7 {
        tss.interrupt_stack_table[i] = Stack::new(65536).rsp();
    }

    // Safety: The tss is stored in the CpuData struct and is not moved
    // or deallocated.
    let tss_descriptor = unsafe { Descriptor::tss_segment_unchecked(&*tss) };

    drop(tss);

    let mut gdt = Box::new(GlobalDescriptorTable::new());
    let kernel_code = gdt.append(Descriptor::kernel_code_segment());
    let kernel_data = gdt.append(Descriptor::kernel_data_segment());
    let user_data = gdt.append(Descriptor::user_data_segment());
    let user_code = gdt.append(Descriptor::user_code_segment());

    let tss_selector = gdt.append(tss_descriptor);

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
        load_tss(tss_selector);
        CS::set_reg(KERNEL_CODE);
        DS::set_reg(KERNEL_DATA);
        ES::set_reg(KERNEL_DATA);
        SS::set_reg(KERNEL_DATA);
    }
}
