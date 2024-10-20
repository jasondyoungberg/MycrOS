use core::{
    alloc::AllocError,
    fmt::Debug,
    sync::atomic::{AtomicU64, Ordering},
};

#[cfg(not(target_arch = "x86_64"))]
compile_error!("this code only works on x86_64");

use bit_field::BitField;
use bitflags::bitflags;

use crate::mem::{phys, MappingKind, Page, PhysPtr};

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

#[derive(Debug)]
#[repr(transparent)]
pub struct PageTableEntry {
    inner: AtomicU64,
}

#[derive(Clone, Copy, Debug)]
pub enum PageTableValue {
    Mapping {
        phys: PhysPtr<Page>,
        flags: PageTableFlags,
    },
    Special(u64),
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PageTableFlags: u64 {
        const PRESENT = 1 << 0;
        const WRITABLE = 1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        const CACHE_DISABLE = 1 << 4;
        const ACCESSED = 1 << 5;
        const HUGE_PAGE = 1 << 7;
        const EXECUTE_DISABLE = 1 << 63;
    }
}

const FLAG_MASK: u64 = 0xFFF0_0000_0000_0FFF;

impl PageTable {
    pub fn new() -> Result<PhysPtr<PageTable>, AllocError> {
        Ok(phys::alloc_zeroed()?.cast())
    }
}

impl PageTableEntry {
    pub fn get(&self) -> Option<PageTableValue> {
        PageTableValue::from_u64(self.inner.load(Ordering::Acquire))
    }

    pub fn get_or_create(&self) -> Result<PageTableValue, AllocError> {
        match self.get() {
            Some(x) => Ok(x),
            None => {
                let pagetable = PageTable::new()?;
                let value = PageTableValue::Mapping {
                    phys: pagetable.cast(),
                    flags: PageTableFlags::from_kind(MappingKind::Full),
                };
                match self.set(value) {
                    Ok(()) => Ok(value),
                    Err(v) => {
                        unsafe { phys::dealloc(pagetable.cast()) };
                        Ok(v)
                    }
                }
            }
        }
    }

    /// # Safety
    /// self must be an entry containing a valid page table
    pub unsafe fn get_pagetable(&self) -> Option<&PageTable> {
        Some(self.get()?.as_pagetable())
    }
    /// # Safety
    /// self must be an entry containing a valid page table
    pub unsafe fn get_pagetable_or_create(&self) -> Result<&PageTable, AllocError> {
        Ok(self.get_or_create()?.as_pagetable())
    }

    pub fn set(&self, value: PageTableValue) -> Result<(), PageTableValue> {
        self.inner
            .compare_exchange(0, value.to_u64(), Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|v| PageTableValue::from_u64(v).expect("v should never be zero"))
    }

    pub fn clear(&self) {
        self.inner.store(0, Ordering::Release);
    }
}

impl PageTableValue {
    pub fn from_u64(value: u64) -> Option<Self> {
        let flags = PageTableFlags::from_bits_retain(value & FLAG_MASK);
        let addr = PhysPtr::new((value & !FLAG_MASK) as usize);

        if value == 0 {
            None
        } else if !flags.contains(PageTableFlags::PRESENT) {
            Some(Self::Special(value >> 1))
        } else {
            Some(Self::Mapping { phys: addr, flags })
        }
    }
    pub fn to_u64(&self) -> u64 {
        match self {
            PageTableValue::Mapping { phys, flags } => {
                (phys.addr() as u64 & !FLAG_MASK) | (flags.bits() & FLAG_MASK)
            }
            PageTableValue::Special(x) => {
                assert!(!x.get_bit(63), "illegal special value");
                *x << 1
            }
        }
    }

    /// # Safety
    /// self must be a value containing a valid page table
    pub unsafe fn as_pagetable<'a>(&self) -> &'a PageTable {
        match self {
            PageTableValue::Mapping { phys, flags } => {
                assert!(
                    !flags.contains(PageTableFlags::HUGE_PAGE),
                    "not a page table"
                );
                phys.cast().as_nonnull().as_ref()
            }
            PageTableValue::Special(_) => panic!("not a page table"),
        }
    }
}

impl PageTableFlags {
    pub fn from_kind(kind: MappingKind) -> Self {
        match kind {
            MappingKind::Code => Self::PRESENT,
            MappingKind::ReadOnly => Self::PRESENT | Self::EXECUTE_DISABLE,
            MappingKind::ReadWrite => Self::PRESENT | Self::WRITABLE | Self::EXECUTE_DISABLE,
            MappingKind::Full => Self::PRESENT | Self::WRITABLE,
            MappingKind::Gaurd => panic!("Gaurd should not be converted to flags"),
            MappingKind::Mmio => {
                Self::PRESENT | Self::WRITABLE | Self::CACHE_DISABLE | Self::EXECUTE_DISABLE
            }
            MappingKind::Framebuffer => {
                // TODO: make write combining
                Self::PRESENT | Self::WRITABLE | Self::CACHE_DISABLE | Self::EXECUTE_DISABLE
            }
        }
    }
    pub fn into_kind(&self) -> Option<MappingKind> {
        if self.intersects(Self::from_kind(MappingKind::Code)) {
            Some(MappingKind::Code)
        } else if self.intersects(Self::from_kind(MappingKind::ReadOnly)) {
            Some(MappingKind::ReadOnly)
        } else if self.intersects(Self::from_kind(MappingKind::ReadWrite)) {
            Some(MappingKind::ReadWrite)
        } else if self.intersects(Self::from_kind(MappingKind::Full)) {
            Some(MappingKind::Full)
        } else if self.intersects(Self::from_kind(MappingKind::Mmio)) {
            Some(MappingKind::Mmio)
        } else if self.intersects(Self::from_kind(MappingKind::Framebuffer)) {
            Some(MappingKind::Framebuffer)
        } else {
            None
        }
    }
}

/// Helper function
pub fn find_pte(ptroot: &PageTable, virt: usize) -> Option<&PageTableEntry> {
    let pt4_index = virt.get_bits(39..48);
    let pt3_index = virt.get_bits(30..39);
    let pt2_index = virt.get_bits(21..30);
    let pt1_index = virt.get_bits(12..21);

    let pt4 = ptroot;

    let pt3 = unsafe { pt4.entries[pt4_index].get_pagetable()? };
    let pt2 = unsafe { pt3.entries[pt3_index].get_pagetable()? };
    let pt1 = unsafe { pt2.entries[pt2_index].get_pagetable()? };

    Some(&pt1.entries[pt1_index])
}

/// Helper function
pub fn find_pte_or_create(ptroot: &PageTable, virt: usize) -> Result<&PageTableEntry, AllocError> {
    let pt4_index = virt.get_bits(39..48);
    let pt3_index = virt.get_bits(30..39);
    let pt2_index = virt.get_bits(21..30);
    let pt1_index = virt.get_bits(12..21);

    let pt4 = ptroot;

    let pt3 = unsafe { pt4.entries[pt4_index].get_pagetable_or_create()? };
    let pt2 = unsafe { pt3.entries[pt3_index].get_pagetable_or_create()? };
    let pt1 = unsafe { pt2.entries[pt2_index].get_pagetable_or_create()? };

    Ok(&pt1.entries[pt1_index])
}
