use core::arch::asm;

use bit_field::BitField;
use spin::{Lazy, Mutex};

use crate::{
    arch::{
        memory::MappingKind,
        pmm::alloc_page_zerod,
        x86_64::structures::paging::{Level4, PageTable, PageTableEntry, PageTableFlags},
        PhysPtr, PAGE_SIZE,
    },
    assert_once,
    boot::memory_mapping,
};

static PML4: Lazy<Mutex<PhysPtr<PageTable<Level4>>>> =
    Lazy::new(|| Mutex::new(alloc_page_zerod().unwrap().cast()));

pub(super) fn init() {
    assert_once!();

    for mapping in memory_mapping() {
        unsafe {
            create_mapping(
                mapping.virt.cast_mut(),
                mapping.phys,
                mapping.size,
                mapping.kind,
            )
        };
    }

    let pml4 = PML4.lock().addr();
    unsafe { asm!("mov cr3, {}", in(reg) pml4) };
}

pub unsafe fn create_mapping(virt: *mut (), phys: PhysPtr<()>, size: usize, kind: MappingKind) {
    let virt_bytes = virt as usize;
    let phys_bytes = phys.addr();

    assert!(
        virt_bytes % PAGE_SIZE == 0,
        "virt ({virt_bytes:#x?}) must be aligned to 4096 bytes"
    );
    assert!(
        phys_bytes % PAGE_SIZE == 0,
        "phys ({phys_bytes:#x?}) must be aligned to 4096 bytes"
    );
    assert!(
        size % PAGE_SIZE == 0,
        "size ({size:#x?}) must be aligned to 4096 bytes"
    );
    assert!(size != 0, "size must be non zero");

    if size > PAGE_SIZE {
        for i in (0..size).step_by(PAGE_SIZE) {
            create_mapping(virt.byte_add(i), phys.byte_add(i), PAGE_SIZE, kind);
        }
        return;
    }

    let pml4_index = virt_bytes.get_bits(39..48);
    let pml3_index = virt_bytes.get_bits(30..39);
    let pml2_index = virt_bytes.get_bits(21..30);
    let pml1_index = virt_bytes.get_bits(12..21);

    let pml4 = PML4.lock().as_mut_ref();

    let pml3 = pml4
        .get_entry_or_create(pml4_index, || {
            PageTableEntry::new(
                alloc_page_zerod().unwrap(),
                PageTableFlags::from_kind(MappingKind::Full),
            )
        })
        .table()
        .as_mut_ref();

    let pml2 = pml3
        .get_entry_or_create(pml3_index, || {
            PageTableEntry::new(
                alloc_page_zerod().unwrap(),
                PageTableFlags::from_kind(MappingKind::Full),
            )
        })
        .table()
        .as_mut_ref();

    let pml1 = pml2
        .get_entry_or_create(pml2_index, || {
            PageTableEntry::new(
                alloc_page_zerod().unwrap(),
                PageTableFlags::from_kind(MappingKind::Full),
            )
        })
        .table()
        .as_mut_ref();

    assert!(
        pml1.get_entry(pml1_index).is_none(),
        "mapping for {virt:#x?} already exists"
    );

    pml1.set_entry(
        pml1_index,
        PageTableEntry::new(phys, PageTableFlags::from_kind(kind)),
    );
}
