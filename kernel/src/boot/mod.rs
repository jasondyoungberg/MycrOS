#[cfg(target_os = "none")]
mod limine;
#[cfg(not(target_os = "none"))]
mod stub;

#[cfg(target_os = "none")]
pub use limine::*;
#[cfg(not(target_os = "none"))]
pub use stub::*;
