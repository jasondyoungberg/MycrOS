use super::PhysPtr;

#[derive(Debug, Clone, Copy)]
pub enum MappingKind {
    Code,
    ReadOnly,
    ReadWrite,
    Full,
    Mmio,
    Framebuffer,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryEntry {
    pub ptr: PhysPtr<()>,
    pub size: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryMapping {
    pub kind: MappingKind,
    pub virt: *const (),
    pub phys: PhysPtr<()>,
    pub size: usize,
}

pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}
