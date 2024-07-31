use core::arch::asm;

fn parse_result(result: u64) -> Result<u64, u64> {
    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall0(n: u64) -> Result<u64, u64> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => result,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        )
    }
    parse_result(result)
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall1(n: u64, arg1: u64) -> Result<u64, u64> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => result,
            in("rdi") arg1,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    parse_result(result)
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall2(n: u64, arg1: u64, arg2: u64) -> Result<u64, u64> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => result,
            in("rdi") arg1,
            in("rsi") arg2,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    parse_result(result)
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall3(n: u64, arg1: u64, arg2: u64, arg3: u64) -> Result<u64, u64> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    parse_result(result)
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall4(n: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> Result<u64, u64> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    parse_result(result)
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall5(
    n: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> Result<u64, u64> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8") arg5,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    parse_result(result)
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall6(
    n: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> Result<u64, u64> {
    let result: u64;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8") arg5,
            in("r9") arg6,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    parse_result(result)
}
