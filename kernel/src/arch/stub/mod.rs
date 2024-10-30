use std::{
    cell::Cell,
    io::{stdout, Write},
};

thread_local! {
    static CPUID: Cell<u32> = panic!();
}

pub fn init(cpuid: u32) {
    CPUID.set(cpuid);
}

pub fn hcf() -> ! {
    panic!("hcf")
}

pub fn get_cpuid() -> u32 {
    CPUID.get()
}

pub fn debug_print(s: &str) {
    stdout().write_all(s.as_bytes()).unwrap();
}
