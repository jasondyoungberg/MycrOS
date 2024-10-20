use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub struct Cr3(pub u64);

impl Cr3 {
    pub fn load() -> Self {
        let res: u64;
        unsafe { asm!("mov {}, cr3", out(reg) res) }
        Self(res)
    }

    pub unsafe fn store(&self) {
        unsafe { asm!("mov cr3, {}", in(reg) self.0) }
    }
}
