#![no_std]

pub mod io;
pub mod syscall;

pub struct FileDescriptor(pub u64);
impl FileDescriptor {
    pub const STDIN: FileDescriptor = FileDescriptor(0);
    pub const STDOUT: FileDescriptor = FileDescriptor(1);
    pub const STDERR: FileDescriptor = FileDescriptor(2);

    /// Reads from the file descriptor into the buffer.
    /// Returns the number of bytes read.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, u64> {
        let ptr = buf.as_mut_ptr() as u64;
        let len = buf.len() as u64;

        unsafe { syscall::syscall3(0, self.0, ptr, len) }.map(|x| x as usize)
    }

    /// Writes the contents of the buffer to the file descriptor.
    /// Returns the number of bytes written.
    pub fn write<T: AsRef<[u8]>>(&self, buf: T) -> Result<usize, u64> {
        let buf = buf.as_ref();
        let ptr = buf.as_ptr() as u64;
        let len = buf.len() as u64;

        unsafe { syscall::syscall3(1, self.0, ptr, len) }.map(|x| x as usize)
    }
}
impl core::fmt::Write for FileDescriptor {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s).map_err(|_| core::fmt::Error).map(|_| ())
    }
}
