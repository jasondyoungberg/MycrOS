use core::{
    array,
    fmt::Debug,
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

use bitflags::bitflags;

use crate::arch::{MappingType, PhysPtr};

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageTable<L: Level> {
    data: [AtomicU64; 512],
    _phantom: PhantomData<L>,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry<L: Level> {
    data: u64,
    _phantom: PhantomData<L>,
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

impl<L: Level> PageTable<L> {
    pub fn new() -> Self {
        PageTable {
            data: array::from_fn(|_| AtomicU64::new(0)),
            _phantom: PhantomData,
        }
    }

    pub fn set_entry(&mut self, index: usize, entry: PageTableEntry<L>) {
        if L::NUM >= 3 {
            assert!(!entry.flags().contains(PageTableFlags::HUGE_PAGE))
        }
        self.data[index].store(entry.data, Ordering::Release);
    }

    pub fn get_entry(&self, index: usize) -> Option<PageTableEntry<L>> {
        PageTableEntry::from_bits(self.data[index].load(Ordering::Acquire))
    }

    pub fn get_entry_or_create(
        &mut self,
        index: usize,
        entry: impl FnOnce() -> PageTableEntry<L>,
    ) -> PageTableEntry<L> {
        self.get_entry(index).unwrap_or_else(|| {
            let entry = entry();
            self.set_entry(index, entry);
            entry
        })
    }
}

impl<L: Level> PageTableEntry<L> {
    const FLAG_MASK: u64 = 0x_FFF0_0000_0000_0FFF;
    const ADDR_MASK: u64 = !Self::FLAG_MASK;

    pub const NULL: Self = PageTableEntry {
        data: 0,
        _phantom: PhantomData,
    };

    pub const fn new(ptr: PhysPtr<()>, flags: PageTableFlags) -> Self {
        assert!(flags.contains(PageTableFlags::PRESENT));
        Self {
            data: flags.bits() & Self::FLAG_MASK | ptr.addr() as u64,
            _phantom: PhantomData,
        }
    }

    pub fn from_bits(data: u64) -> Option<Self> {
        if PageTableFlags::from_bits_retain(data).contains(PageTableFlags::PRESENT) {
            Some(Self {
                data,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }

    pub const fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_retain(self.data & Self::FLAG_MASK)
    }

    pub const fn ptr(&self) -> PhysPtr<()> {
        PhysPtr::new((self.data & Self::ADDR_MASK) as usize)
    }
}

impl<L: LevelDec> PageTableEntry<L> {
    pub fn table(&self) -> PhysPtr<PageTable<L::Dec>> {
        self.ptr().cast()
    }
}

impl PageTableFlags {
    pub fn from_kind(kind: MappingType) -> Self {
        match kind {
            MappingType::Code => Self::PRESENT,
            MappingType::ReadOnly => Self::PRESENT | Self::EXECUTE_DISABLE,
            MappingType::ReadWrite => Self::PRESENT | Self::WRITABLE | Self::EXECUTE_DISABLE,
            MappingType::Mmio => {
                Self::PRESENT | Self::WRITABLE | Self::CACHE_DISABLE | Self::EXECUTE_DISABLE
            }
            MappingType::Framebuffer => {
                Self::PRESENT | Self::WRITABLE | Self::CACHE_DISABLE | Self::EXECUTE_DISABLE
            }
        }
    }
}

impl<L: Level> Debug for PageTableEntry<L> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageTableEntry")
            .field("level", &L::NUM)
            .field("ptr", &self.ptr())
            .field("flags", &self.flags())
            .finish()
    }
}

// Type System Shenanigans

pub trait Level: Debug + Clone + Copy {
    const NUM: u8;
}
pub trait LevelDec: Level {
    type Dec: Level;
}

#[derive(Debug, Clone, Copy)]
pub struct Level4;
impl Level for Level4 {
    const NUM: u8 = 4;
}
impl LevelDec for Level4 {
    type Dec = Level3;
}

#[derive(Debug, Clone, Copy)]
pub struct Level3;
impl Level for Level3 {
    const NUM: u8 = 3;
}
impl LevelDec for Level3 {
    type Dec = Level2;
}

#[derive(Debug, Clone, Copy)]
pub struct Level2;
impl Level for Level2 {
    const NUM: u8 = 2;
}
impl LevelDec for Level2 {
    type Dec = Level1;
}

#[derive(Debug, Clone, Copy)]
pub struct Level1;
impl Level for Level1 {
    const NUM: u8 = 1;
}
