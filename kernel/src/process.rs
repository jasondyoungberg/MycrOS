use core::{
    arch::asm,
    fmt::Debug,
    sync::atomic::{AtomicU64, Ordering},
};

use alloc::{
    collections::{BTreeMap, VecDeque},
    string::{String, ToString},
    sync::Arc,
};
use spin::Mutex;
use x86_64::{structures::paging::PhysFrame, VirtAddr};

use crate::{mapper::create_l4_table, proot::proot, stack::Stack};

pub static MANAGER: Manager = Manager::new();

pub struct Manager {
    processes: Mutex<BTreeMap<ProcessId, Arc<Mutex<Process>>>>,
    queue: Mutex<VecDeque<Arc<Mutex<Process>>>>,
}

#[derive(Debug)]
pub struct Process {
    pid: ProcessId,
    name: String,
    state: ProcessState,

    l4_table: PhysFrame,
    kernel_stack: Stack,
}

#[derive(Debug)]
pub enum ProcessState {
    Paused { rsp: VirtAddr },
    Running,
    Terminated(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(u64);

impl Manager {
    pub const fn new() -> Self {
        Self {
            processes: Mutex::new(BTreeMap::new()),
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn get_pid(&self, pid: ProcessId) -> Option<Arc<Mutex<Process>>> {
        self.processes.lock().get(&pid).cloned()
    }

    fn add_process(&self, process: Process) {
        let process = Arc::new(Mutex::new(process));

        self.processes
            .lock()
            .insert(process.lock().pid, process.clone());
        self.queue.lock().push_back(process);
    }

    pub fn init(&self) {
        let kernel_process =
            Process::new("proot".to_string(), VirtAddr::from_ptr(proot as *const ()));

        self.add_process(kernel_process);
    }

    /// Try to start on the next process in the queue (if there is one)
    pub fn get_process(&self) {
        if let Some(new_process) = self.queue.lock().pop_front() {
            let ProcessState::Paused { rsp } = new_process.lock().state else {
                panic!("Process should be paused")
            };

            new_process.lock().state = ProcessState::Running;

            // Safety: We are switching to a new process, so it's safe to change the stack pointer
            unsafe {
                asm!(
                    "mov rsp, {0}",
                    "ret",
                    in(reg) rsp.as_u64(),
                    options(noreturn)
                )
            }
        }
    }
}

impl Process {
    fn new(name: String, instruction_pointer: VirtAddr) -> Self {
        Self::new_pid(name, instruction_pointer, ProcessId::new())
    }

    fn new_pid(name: String, instruction_pointer: VirtAddr, pid: ProcessId) -> Self {
        let kernel_stack = Stack::new(65536);

        let stack_base = kernel_stack.rsp().as_mut_ptr::<VirtAddr>();

        // Safety: We just allocated this stack, so it's safe to write to it
        unsafe {
            *stack_base.sub(1) = VirtAddr::zero();
            *stack_base.sub(2) = instruction_pointer;
        }

        Self {
            pid,
            name,
            state: ProcessState::Paused {
                rsp: kernel_stack.rsp() - 16,
            },
            l4_table: create_l4_table(),
            kernel_stack,
        }
    }
}

impl ProcessId {
    pub fn new() -> Self {
        static NEXT_PID: AtomicU64 = AtomicU64::new(1);

        Self(NEXT_PID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Debug for Manager {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Manager")
            .field("processes", &self.processes.lock())
            .field("queue", &self.queue.lock())
            .finish()
    }
}
