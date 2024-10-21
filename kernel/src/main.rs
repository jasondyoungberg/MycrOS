#![no_std]
#![no_main]
#![feature(allocator_api)]
#![allow(dead_code)]

extern crate alloc;

pub mod boot;
pub mod cpulocal;
pub mod debug;
pub mod framebuffer;
mod heap;
pub mod mem;
mod print;
pub mod stack;
mod x86_64;

use core::arch::asm;

use cpulocal::CpuLocal;
use framebuffer::FRAMEBUFFER;

#[no_mangle]
extern "C" fn entry() -> ! {
    boot::verify();

    println!("Hello, World!");

    for (i, c) in "Hello, World!".chars().enumerate() {
        FRAMEBUFFER.lock().draw_char(c, (i, 0));
    }

    boot::smp_init();
}

#[no_mangle]
extern "C" fn smp_main() -> ! {
    println!("I'm cpu {}", *CPUID);
    println!("goodbye");
    hcf();
}

static CPUID: CpuLocal<u32> = CpuLocal::new(|id| id);

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
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
