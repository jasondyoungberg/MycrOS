use core::ptr::addr_of;

use limine::{
    memory_map::EntryType,
    request::{HhdmRequest, KernelAddressRequest, KernelFileRequest, MemoryMapRequest},
    BaseRevision,
};

use crate::arch::{
    memory::{MappingKind, MemoryEntry, MemoryMapping},
    PhysPtr,
};

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".requests"]
static KERNEL_ADDRESS_REQUEST: KernelAddressRequest = KernelAddressRequest::new();

#[used]
#[link_section = ".requests"]
static KERNEL_FILE_REQUEST: KernelFileRequest = KernelFileRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
];

#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];

pub fn verify() {
    assert!(BASE_REVISION.is_supported());
    assert!(MEMORY_MAP_REQUEST.get_response().is_some());
    assert!(HHDM_REQUEST.get_response().is_some());
    assert!(KERNEL_ADDRESS_REQUEST.get_response().is_some());
    assert!(KERNEL_FILE_REQUEST.get_response().is_some());
}

pub fn memory_map() -> impl Iterator<Item = MemoryEntry> {
    MEMORY_MAP_REQUEST
        .get_response()
        .unwrap()
        .entries()
        .iter()
        .filter(|e| e.entry_type == EntryType::USABLE)
        .map(|e| MemoryEntry {
            ptr: PhysPtr::new(e.base as usize),
            size: e.length as usize,
        })
}

pub fn hhdm_offset() -> PhysPtr<()> {
    PhysPtr::new(HHDM_REQUEST.get_response().unwrap().offset() as usize)
}

pub fn memory_mapping() -> impl Iterator<Item = MemoryMapping> {
    extern "C" {
        static elf_start: u8;
        static text_start: u8;
        static text_end: u8;
        static rodata_start: u8;
        static rodata_end: u8;
        static data_start: u8;
        static data_end: u8;
        static elf_end: u8;
    }

    let elf_start_addr = addr_of!(elf_start) as usize;
    let text_start_addr = addr_of!(text_start) as usize;
    let text_end_addr = addr_of!(text_end) as usize;
    let rodata_start_addr = addr_of!(rodata_start) as usize;
    let rodata_end_addr = addr_of!(rodata_end) as usize;
    let data_start_addr = addr_of!(data_start) as usize;
    let data_end_addr = addr_of!(data_end) as usize;
    let _elf_end_addr = addr_of!(elf_end) as usize;

    let address = KERNEL_ADDRESS_REQUEST.get_response().unwrap();
    let kernel_virt = address.virtual_base() as usize;
    let kernel_phys = PhysPtr::new(address.physical_base() as usize);

    let kaslr_offset = kernel_virt - elf_start_addr;

    let text_mapping = MemoryMapping {
        kind: MappingKind::Code,
        virt: (text_start_addr + kaslr_offset) as *const (),
        phys: kernel_phys.byte_add(text_start_addr - elf_start_addr),
        size: text_end_addr - text_start_addr,
    };
    let rodata_mapping = MemoryMapping {
        kind: MappingKind::ReadOnly,
        virt: (rodata_start_addr + kaslr_offset) as *const (),
        phys: kernel_phys.byte_add(rodata_start_addr - elf_start_addr),
        size: rodata_end_addr - rodata_start_addr,
    };
    let data_mapping = MemoryMapping {
        kind: MappingKind::ReadWrite,
        virt: (data_start_addr + kaslr_offset) as *const (),
        phys: kernel_phys.byte_add(data_start_addr - elf_start_addr),
        size: data_end_addr - data_start_addr,
    };

    MEMORY_MAP_REQUEST
        .get_response()
        .unwrap()
        .entries()
        .iter()
        .filter_map(|e| {
            let phys = PhysPtr::new(e.base as usize);
            let virt = phys.as_ptr();
            let size = e.length as usize;

            match e.entry_type {
                EntryType::USABLE
                | EntryType::ACPI_RECLAIMABLE
                | EntryType::BOOTLOADER_RECLAIMABLE
                | EntryType::KERNEL_AND_MODULES => Some(MemoryMapping {
                    kind: MappingKind::ReadWrite,
                    virt,
                    phys,
                    size,
                }),
                EntryType::ACPI_NVS => Some(MemoryMapping {
                    kind: MappingKind::Mmio,
                    virt,
                    phys,
                    size,
                }),
                EntryType::FRAMEBUFFER => Some(MemoryMapping {
                    kind: MappingKind::Framebuffer,
                    virt,
                    phys,
                    size,
                }),
                _ => None,
            }
        })
        .chain([text_mapping, rodata_mapping, data_mapping])
}
