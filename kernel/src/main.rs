#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![feature(allocator_api)]
#![allow(dead_code)]

extern crate alloc;

pub mod arch;
pub mod boot;
pub mod cpulocal;
pub mod framebuffer;
pub mod heap;
pub mod mem;
pub mod print;
pub mod stack;

use cpulocal::CpuLocal;

#[no_mangle]
extern "C" fn kmain() -> ! {
    println!("I'm cpu {}", *CPUID);
    println!("goodbye");
    arch::hcf();
}

static CPUID: CpuLocal<u32> = CpuLocal::new(|id| id);

#[cfg_attr(target_os = "none", panic_handler)]
fn _rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    arch::hcf();
}

#[cfg(not(target_os = "none"))]
fn main() {
    boot::entry();
}

#[macro_export]
macro_rules! assert_once {
    () => {
        static CALLED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
        CALLED
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            )
            .expect("this function may only be called once");
    };
}

#[macro_export]
macro_rules! assert_once_percpu {
    () => {
        assert_once_percpu!($crate::cpulocal::get_percpu().cpuid)
    };
    ($cpuid: expr) => {
        let cpuid: u32 = ($cpuid);
        static CALLED: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
        if cpuid < 64 {
            let prev = CALLED.fetch_and(1 << cpuid, core::sync::atomic::Ordering::AcqRel);
            if prev & (1 << cpuid) != 0 {
                panic!("this function may only be called once per cpu")
            }
        }
    };
}
