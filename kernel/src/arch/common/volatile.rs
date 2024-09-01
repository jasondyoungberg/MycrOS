use core::{
    cell::UnsafeCell,
    ptr::{self, addr_of, addr_of_mut},
};

#[repr(transparent)]
pub struct Volatile<T> {
    data: T,
}

impl<T> Volatile<T> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(addr_of!(self.data)) }
    }

    pub fn write(&mut self, value: T) {
        unsafe { ptr::write_volatile(addr_of_mut!(self.data), value) }
    }
}
