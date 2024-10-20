pub mod x86_64;

use core::{alloc::AllocError, array};

use spin::{Lazy, Mutex};
use x86_64::{find_pte, PageTable, PageTableFlags, PageTableValue};

use crate::{boot::virt_memmap, mem::HIGHER_HALF_ADDR, println};

use super::{phys, Page, PhysPtr, PAGE_SIZE};

#[derive(Debug, Clone, Copy)]
pub enum MappingKind {
    /// perms: read/execute
    Code,
    /// perms: r--
    ReadOnly,
    /// perms: rw-
    ReadWrite,
    /// perms: rwx
    Full,
    /// perms: ---
    Gaurd,
    /// perms: rw- (no caching)
    Mmio,
    /// perms: -w- (write combining)
    Framebuffer,
}

#[derive(Debug, Clone, Copy)]
pub struct MappingInfo {
    pub virt: *const Page,
    pub phys: Option<PhysPtr<Page>>,
    pub kind: MappingKind,
}

#[derive(Debug, Clone, Copy)]
pub enum MappingError {
    AlreadyMapped,
    AllocError(AllocError),
}

impl MappingKind {
    pub fn can_read(&self) -> bool {
        match self {
            MappingKind::Code => true,
            MappingKind::ReadOnly => true,
            MappingKind::ReadWrite => true,
            MappingKind::Full => true,
            MappingKind::Gaurd => false,
            MappingKind::Mmio => true,
            MappingKind::Framebuffer => false,
        }
    }
    pub fn can_write(&self) -> bool {
        match self {
            MappingKind::Code => false,
            MappingKind::ReadOnly => false,
            MappingKind::ReadWrite => true,
            MappingKind::Full => true,
            MappingKind::Gaurd => false,
            MappingKind::Mmio => true,
            MappingKind::Framebuffer => true,
        }
    }
    pub fn can_execute(&self) -> bool {
        match self {
            MappingKind::Code => true,
            MappingKind::ReadOnly => false,
            MappingKind::ReadWrite => false,
            MappingKind::Full => false,
            MappingKind::Gaurd => false,
            MappingKind::Mmio => false,
            MappingKind::Framebuffer => false,
        }
    }
}

pub struct KernelMapper {
    ptroot: &'static PageTable,
    phys: PhysPtr<PageTable>,
}

pub trait Mapper {
    /// Maps a virtual address range
    /// # Errors
    /// - [MappingError::Misaligned] if either `ptr` or `size` are not aligned to [PAGE_SIZE]
    /// - [MappingError::AlreadyMapped] if any part of the mapping is already mapped
    /// - [MappingError::AllocError] if there was an issue allocating page tables
    /// # Safety
    /// Memory managment is fundumentally unsafe
    unsafe fn map(
        &mut self,
        ptr: *mut (),
        size: usize,
        kind: MappingKind,
    ) -> Result<(), MappingError>;

    /// Maps a virtual address range, filling it with zeros
    /// # Errors
    /// - [MappingError::Misaligned] if either `ptr` or `size` are not aligned to [PAGE_SIZE]
    /// - [MappingError::AlreadyMapped] if any part of the mapping is already mapped
    /// - [MappingError::AllocError] if there was an issue allocating page tables
    /// # Safety
    /// Memory managment is fundumentally unsafe
    unsafe fn map_zeroed(
        &mut self,
        ptr: *mut (),
        size: usize,
        kind: MappingKind,
    ) -> Result<(), MappingError>;

    /// Maps a virtual address range to a physical address range
    /// # Errors
    /// - [MappingError::Misaligned] if either `ptr` or `size` are not aligned to [PAGE_SIZE]
    /// - [MappingError::AlreadyMapped] if any part of the mapping is already mapped
    /// - [MappingError::AllocError] if there was an issue allocating page tables
    /// # Safety
    /// Memory managment is fundumentally unsafe
    unsafe fn map_phys(
        &mut self,
        virt: *mut (),
        phys: PhysPtr<()>,
        size: usize,
        kind: MappingKind,
    ) -> Result<(), MappingError>;

    /// Unmaps a virtual address range
    /// # Safety
    /// The address range must be unused
    unsafe fn unmap(&mut self, ptr: *mut (), size: usize);

    /// Queries the page tables for info about an address
    fn query(&mut self, ptr: *const ()) -> Option<MappingKind>;

    fn ptroot(&self) -> PhysPtr<PageTable>;
}

pub static KERNEL_MAPPER: Lazy<Mutex<KernelMapper>> = Lazy::new(|| Mutex::new(KernelMapper::new()));

struct DefaultEntries([PageTableValue; 256]);
unsafe impl Sync for DefaultEntries {}

static DEFAULT_ENTRIES: Lazy<DefaultEntries> = Lazy::new(|| {
    DefaultEntries(array::from_fn(|_| PageTableValue::Mapping {
        phys: phys::alloc_zeroed().expect("critical allocation failed"),
        flags: PageTableFlags::from_kind(MappingKind::Full),
    }))
});

const SPECIAL_GAURD: u64 = 1;

impl KernelMapper {
    fn new() -> Self {
        println!("init vmm");

        let phys = PageTable::new().expect("critical allocation failed");
        let ptroot = unsafe { phys.as_nonnull().as_ref() };

        for i in 0..256 {
            ptroot.entries[i + 256]
                .set(DEFAULT_ENTRIES.0[i])
                .expect("ptroot should be empty");
        }

        let mut res = Self { ptroot, phys };

        for entry in virt_memmap() {
            unsafe { res.map_phys(entry.virt.cast_mut(), entry.phys, entry.size, entry.kind) }
                .expect("critical mapping failed");
        }

        println!("vmm ready");
        res
    }
}

impl Mapper for KernelMapper {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    unsafe fn map(
        &mut self,
        ptr: *mut (),
        size: usize,
        kind: MappingKind,
    ) -> Result<(), MappingError> {
        let vaddr = ptr as usize;
        assert!(vaddr >= HIGHER_HALF_ADDR, "ptr is not in higher half");
        assert!(vaddr % PAGE_SIZE == 0, "ptr is misaligned");
        assert!(size % PAGE_SIZE == 0, "size is misaligned");

        for i in (0..size).step_by(PAGE_SIZE) {
            let pte = x86_64::find_pte_or_create(self.ptroot, vaddr + i);
            pte.map_err(|e| {
                unsafe { self.unmap(ptr, i.saturating_sub(PAGE_SIZE)) }
                MappingError::AllocError(e)
            })?
            .set(match kind {
                MappingKind::Gaurd => PageTableValue::Special(SPECIAL_GAURD),
                _ => PageTableValue::Mapping {
                    phys: phys::alloc().map_err(|e| {
                        unsafe { self.unmap(ptr, i.saturating_sub(PAGE_SIZE)) }
                        MappingError::AllocError(e)
                    })?,
                    flags: PageTableFlags::from_kind(kind),
                },
            })
            .map_err(|_| {
                unsafe { self.unmap(ptr, i.saturating_sub(PAGE_SIZE)) }
                MappingError::AlreadyMapped
            })?
        }

        Ok(())
    }

    unsafe fn map_zeroed(
        &mut self,
        ptr: *mut (),
        size: usize,
        kind: MappingKind,
    ) -> Result<(), MappingError> {
        let vaddr = ptr as usize;

        assert!(vaddr >= HIGHER_HALF_ADDR, "ptr is not in higher half");
        assert!(vaddr % PAGE_SIZE == 0, "ptr is misaligned");
        assert!(size % PAGE_SIZE == 0, "size is misaligned");

        for i in (0..size).step_by(PAGE_SIZE) {
            let pte = x86_64::find_pte_or_create(self.ptroot, vaddr + i);
            pte.map_err(|e| {
                unsafe { self.unmap(ptr, i.saturating_sub(PAGE_SIZE)) }
                MappingError::AllocError(e)
            })?
            .set(match kind {
                MappingKind::Gaurd => PageTableValue::Special(SPECIAL_GAURD),
                _ => PageTableValue::Mapping {
                    phys: phys::alloc_zeroed().map_err(|e| {
                        unsafe { self.unmap(ptr, i.saturating_sub(PAGE_SIZE)) }
                        MappingError::AllocError(e)
                    })?,
                    flags: PageTableFlags::from_kind(kind),
                },
            })
            .map_err(|_| {
                unsafe { self.unmap(ptr, i.saturating_sub(PAGE_SIZE)) }
                MappingError::AlreadyMapped
            })?
        }

        Ok(())
    }

    unsafe fn map_phys(
        &mut self,
        virt: *mut (),
        phys: PhysPtr<()>,
        size: usize,
        kind: MappingKind,
    ) -> Result<(), MappingError> {
        let vaddr = virt as usize;
        assert!(vaddr >= HIGHER_HALF_ADDR, "virt is not in higher half");
        assert!(vaddr % PAGE_SIZE == 0, "virt is misaligned");
        assert!(size % PAGE_SIZE == 0, "size is misaligned");
        assert!(phys.addr() % PAGE_SIZE == 0, "phys is misaligned");

        for i in (0..size).step_by(PAGE_SIZE) {
            let pte = x86_64::find_pte_or_create(self.ptroot, vaddr + i);
            pte.map_err(|e| {
                unsafe { self.unmap(virt, i.saturating_sub(PAGE_SIZE)) }
                MappingError::AllocError(e)
            })?
            .set(PageTableValue::Mapping {
                phys: phys.cast().byte_add(i),
                flags: PageTableFlags::from_kind(kind),
            })
            .map_err(|_| {
                unsafe { self.unmap(virt, i.saturating_sub(PAGE_SIZE)) }
                MappingError::AlreadyMapped
            })?
        }

        Ok(())
    }

    unsafe fn unmap(&mut self, ptr: *mut (), size: usize) {
        let vaddr = ptr as usize;

        assert!(vaddr >= HIGHER_HALF_ADDR, "ptr is not in higher half");
        assert!(vaddr % PAGE_SIZE == 0, "ptr is misaligned");
        assert!(size % PAGE_SIZE == 0, "size is misaligned");

        for i in (0..size).step_by(PAGE_SIZE) {
            if let Some(pte) = x86_64::find_pte(self.ptroot, vaddr + i) {
                pte.clear();
            }
        }

        // todo: tlb shootdown
    }

    fn query(&mut self, ptr: *const ()) -> Option<MappingKind> {
        Some(match find_pte(self.ptroot, ptr as usize)?.get()? {
            PageTableValue::Mapping { flags, .. } => {
                flags.into_kind().expect("flags should be a known kind")
            }
            PageTableValue::Special(SPECIAL_GAURD) => MappingKind::Gaurd,
            PageTableValue::Special(_) => panic!("invalid special value"),
        })
    }

    fn ptroot(&self) -> PhysPtr<PageTable> {
        self.phys
    }
}
