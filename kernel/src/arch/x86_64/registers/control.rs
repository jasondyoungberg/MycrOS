use core::arch::asm;

pub struct Cr0;
impl Cr0 {
    pub fn read_raw() -> u64 {
        let res: u64;
        unsafe { asm!("mov {}, cr0", out(reg) res, options(nomem, nostack, preserves_flags)) }
        res
    }

    pub unsafe fn write_raw(val: u64) {
        unsafe { asm!("mov cr3, {}", in(reg) val, options(nostack, preserves_flags)) }
    }
}

pub struct Cr2;
impl Cr2 {
    pub fn read_raw() -> u64 {
        let res: u64;
        unsafe { asm!("mov {}, cr2", out(reg) res, options(nomem, nostack, preserves_flags)) }
        res
    }

    pub unsafe fn write_raw(val: u64) {
        unsafe { asm!("mov cr2, {}", in(reg) val, options(nostack, preserves_flags)) }
    }
}

pub struct Cr3;
impl Cr3 {
    pub fn read_raw() -> u64 {
        let res: u64;
        unsafe { asm!("mov {}, cr3", out(reg) res, options(nomem, nostack, preserves_flags)) }
        res
    }

    pub unsafe fn write_raw(val: u64) {
        unsafe { asm!("mov cr3, {}", in(reg) val, options(nostack, preserves_flags)) }
    }
}

pub struct Cr4;
impl Cr4 {
    pub fn read_raw() -> u64 {
        let res: u64;
        unsafe { asm!("mov {}, cr4", out(reg) res, options(nomem, nostack, preserves_flags)) }
        res
    }

    pub unsafe fn write_raw(val: u64) {
        unsafe { asm!("mov cr4, {}", in(reg) val, options(nostack, preserves_flags)) }
    }
}
