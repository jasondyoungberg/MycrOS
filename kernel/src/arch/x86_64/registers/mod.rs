pub mod control;
pub mod model_specific;

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

pub unsafe fn write_msr(id: u32) -> u64 {
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
