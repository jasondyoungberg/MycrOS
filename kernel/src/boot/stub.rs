use std::{iter, thread};

use crate::{
    arch, kmain,
    mem::{MappingKind, PhysPtr},
};

#[derive(Debug)]
pub struct MemoryMapping {
    pub kind: MappingKind,
    pub virt: *const (),
    pub phys: PhysPtr<()>,
    pub size: usize,
}

const CPUS: u32 = 4;

pub fn entry() -> ! {
    fn smp_entry(cpuid: u32) -> ! {
        arch::init(cpuid);
        kmain();
    }

    for i in 1..cpu_count() {
        thread::spawn(move || smp_entry(i));
    }
    smp_entry(0);
}

pub fn cpu_count() -> u32 {
    CPUS
}

pub fn phys_memmap_usable() -> impl Iterator<Item = (PhysPtr<()>, usize)> {
    todo!();
    #[expect(unreachable_code)]
    iter::empty()
}

pub fn hhdm_offset() -> *const () {
    todo!()
}

pub fn virt_memmap() -> impl Iterator<Item = MemoryMapping> {
    todo!();
    #[expect(unreachable_code)]
    iter::empty()
}
