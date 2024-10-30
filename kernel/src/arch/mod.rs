#[cfg(not(target_os = "none"))]
mod stub;
#[cfg(all(target_arch = "x86_64", target_os = "none"))]
mod x86_64;

#[cfg(not(target_os = "none"))]
pub use stub::*;
#[cfg(all(target_arch = "x86_64", target_os = "none"))]
pub use x86_64::*;
