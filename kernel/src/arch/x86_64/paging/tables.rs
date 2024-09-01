use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::arch::PhysPtr;

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageTable<L: Level> {
    data: [PageTableEntry<L>; 512],
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PageTableEntry<L: Level> {
    data: u64,
    _phantom: PhantomData<L>,
}

impl<L: Level> PageTable<L> {
    pub const fn new() -> Self {
        PageTable {
            data: [PageTableEntry::NULL; 512],
        }
    }
}

impl<L: Level> Deref for PageTable<L> {
    type Target = [PageTableEntry<L>; 512];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<L: Level> DerefMut for PageTable<L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<L: Level> PageTableEntry<L> {
    pub const NULL: Self = PageTableEntry {
        data: 0,
        _phantom: PhantomData,
    };
}

impl<L: Level + LevelDec> PageTableEntry<L> {
    pub const fn read_table(&self) -> PhysPtr<PageTable<L::Dec>> {
        PhysPtr::new((self.data & 0x000F_FFFF_FFFF_F000) as usize)
    }
}

impl PageTableEntry<Level1> {
    pub const fn read(&self) -> PhysPtr<()> {
        PhysPtr::new((self.data & 0x000F_FFFF_FFFF_F000) as usize)
    }
}

// Type System Shenanigans

pub trait Level {
    const NUM: u8;
}
pub trait LevelDec {
    type Dec: Level;
}

#[derive(Debug)]
pub struct Level4;
impl Level for Level4 {
    const NUM: u8 = 4;
}
impl LevelDec for Level4 {
    type Dec = Level3;
}

#[derive(Debug)]
pub struct Level3;
impl Level for Level3 {
    const NUM: u8 = 3;
}
impl LevelDec for Level3 {
    type Dec = Level2;
}

#[derive(Debug)]
pub struct Level2;
impl Level for Level2 {
    const NUM: u8 = 2;
}
impl LevelDec for Level2 {
    type Dec = Level1;
}

#[derive(Debug)]
pub struct Level1;
impl Level for Level1 {
    const NUM: u8 = 1;
}
