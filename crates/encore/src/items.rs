/// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

/// Exception handling personality
#[lang = "eh_personality"]
fn eh_personality() {}

// Provides memset, memcpy
extern crate rlibc;

/// Used by parts of libcore that aren't panic=abort
#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C" fn _Unwind_Resume() {}

/// Provides bcmp (aliased below)
extern crate compiler_builtins;

#[no_mangle]
unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    compiler_builtins::mem::bcmp(s1, s2, n)
}