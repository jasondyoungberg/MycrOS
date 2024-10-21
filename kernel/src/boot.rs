use core::{arch::asm, ptr::addr_of};

use limine::{
    memory_map::EntryType,
    request::{
        HhdmRequest, KernelAddressRequest, KernelFileRequest, MemoryMapRequest, RequestsEndMarker,
        RequestsStartMarker, SmpRequest, StackSizeRequest,
    },
    smp::Cpu,
    BaseRevision,
};

use crate::{
    mem::{MappingKind, PhysPtr},
    println, smp_main,
    stack::{Stack, STACK_SIZE},
    x86_64,
};

#[derive(Debug)]
pub struct MemoryMapping {
    pub kind: MappingKind,
    pub virt: *const (),
    pub phys: PhysPtr<()>,
    pub size: usize,
}

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
#[link_section = ".requests"]
static SMP_REQUEST: SmpRequest = SmpRequest::new();

#[used]
#[link_section = ".requests"]
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(STACK_SIZE as u64);

#[used]
#[link_section = ".requests_start_marker"]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".requests_end_marker"]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

pub fn verify() {
    assert!(BASE_REVISION.is_supported());
    assert!(MEMORY_MAP_REQUEST.get_response().is_some());
    assert!(HHDM_REQUEST.get_response().is_some());
    assert!(KERNEL_ADDRESS_REQUEST.get_response().is_some());
    assert!(KERNEL_FILE_REQUEST.get_response().is_some());
    assert!(SMP_REQUEST.get_response().is_some());
}

pub fn smp_init() -> ! {
    extern "C" fn entry(cpu: &Cpu) -> ! {
        let response = SMP_REQUEST.get_response().unwrap();
        let bsp_lapic_id = response.bsp_lapic_id();
        let cpus = response.cpus();

        x86_64::init();

        let cpuid = if cpu.lapic_id == bsp_lapic_id {
            0
        } else {
            cpus.iter()
                .filter(|c| c.lapic_id != bsp_lapic_id)
                .enumerate()
                .find(|(_, c)| c.lapic_id == cpu.lapic_id)
                .unwrap()
                .0
                + 1
        };

        let new_sp = Stack::new()
            .expect("critical mapping failed")
            .stack_pointer();

        unsafe {
            asm!(
                "
            mov rsp, {new_sp}
            mov rdi, {cpuid}
            push 0
            jmp {smp_main}
        ",
                new_sp = in(reg) new_sp,
                cpuid = in(reg) cpuid,
                smp_main = sym smp_main,
                options(noreturn)
            )
        }
    }

    println!("init smp");

    let response = SMP_REQUEST.get_response().unwrap();
    let bsp_lapic_id = response.bsp_lapic_id();
    let cpus = response.cpus();

    cpus.iter()
        .filter(|c| c.lapic_id != bsp_lapic_id)
        .for_each(|c| c.goto_address.write(entry));

    entry(
        cpus.iter()
            .find(|c| c.lapic_id == bsp_lapic_id)
            .expect("there should be a bsp"),
    )
}

pub fn cpu_count() -> usize {
    SMP_REQUEST.get_response().unwrap().cpus().len()
}

pub fn phys_memmap_usable() -> impl Iterator<Item = (PhysPtr<()>, usize)> {
    MEMORY_MAP_REQUEST
        .get_response()
        .unwrap()
        .entries()
        .iter()
        .filter(|e| e.entry_type == EntryType::USABLE)
        .map(|e| (PhysPtr::new(e.base as usize), e.length as usize))
}

pub fn hhdm_offset() -> *const () {
    HHDM_REQUEST.get_response().unwrap().offset() as usize as *const ()
}

pub fn virt_memmap() -> impl Iterator<Item = MemoryMapping> {
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
