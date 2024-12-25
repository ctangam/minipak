use core::arch::asm;

/// A file descriptor
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileDescriptor(pub u64);

impl FileDescriptor {
    /// Standard input file descriptor
    pub const STDIN: Self = Self(0);
    /// Standard output file descriptor
    pub const STDOUT: Self = Self(1);
    /// Standard error file descriptor
    pub const STDERR: Self = Self(2);
}

/// # Safety
/// Calls into the kernel.
#[inline(always)]
pub unsafe fn write(fd: FileDescriptor, buf: *const u8, count: u64) -> u64 {
    let syscall_number: u64 = 1;
    let mut rax = syscall_number;

    asm!(
        "syscall",
        inout("rax") rax,
        in("rdi") fd.0,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _, lateout("r11") _,
        options(nostack),
    );
    rax
}
