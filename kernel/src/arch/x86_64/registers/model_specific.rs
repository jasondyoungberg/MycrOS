use core::arch::asm;

pub unsafe fn read_msr(id: u32) -> u64 {
    let (high, low): (u32, u32);
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") id,
            out("eax") low, out("edx") high,
            options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}

pub unsafe fn write_msr(id: u32, val: u64) {
    let low = val as u32;
    let high = (val >> 32) as u32;

    unsafe {
        asm!(
            "wrmsr",
            in("ecx") id,
            in("eax") low, in("edx") high,
            options(nostack, preserves_flags),
        );
    }
}

pub struct Msr<const N: u32>;
impl<const N: u32> Msr<N> {
    pub unsafe fn read_raw() -> u64 {
        read_msr(N)
    }
    pub unsafe fn write_raw(val: u64) {
        write_msr(N, val)
    }
}

pub type FsBase = Msr<0xC000_0100>;
pub type GsBase = Msr<0xC000_0101>;
pub type KernelGsBase = Msr<0xC000_0102>;
