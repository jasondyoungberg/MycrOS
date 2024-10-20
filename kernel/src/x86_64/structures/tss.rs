use core::ptr;

#[repr(C, packed(4))]
pub struct TaskStateSegment {
    _reserved_1: u32,
    pub privilege_stack_table: [*mut (); 3],
    _reserved_2: u64,
    pub interrupt_stack_table: [*mut (); 7],
    _reserved_3: u64,
    _reserved_4: u16,
    pub iomap_base: u16,
}

impl TaskStateSegment {
    pub fn new() -> Self {
        Self {
            _reserved_1: 0,
            privilege_stack_table: [ptr::null_mut(); 3],
            _reserved_2: 0,
            interrupt_stack_table: [ptr::null_mut(); 7],
            _reserved_3: 0,
            _reserved_4: 0,
            iomap_base: size_of::<Self>().try_into().unwrap(),
        }
    }
}
