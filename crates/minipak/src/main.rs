// Opt out of libstd
#![no_std]
// Let us worry about the entry point.
#![no_main]
// Let us make functions without any prologue - assembly only!
#![feature(naked_functions)]
#![feature(link_arg_attribute)]

#[allow(unused_attributes)]
#[link(kind = "link-arg", name = "-nostartfiles", modifiers = "+verbatim")]
#[link(kind = "link-arg", name = "-nodefaultlibs", modifiers = "+verbatim")]
#[link(kind = "link-arg", name = "-static", modifiers = "+verbatim")]
unsafe extern "C" {}

use core::arch::naked_asm;

#[naked]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() {
    unsafe { naked_asm!("mov rdi, rsp", "call pre_main") }
}

#[unsafe(no_mangle)]
unsafe fn pre_main(_stack_top: *mut u8) {
    let s = "Hello from minipak!\n";
    unsafe { encore::syscall::write(
        encore::syscall::FileDescriptor::STDOUT,
        s.as_bytes().as_ptr(),
        s.len() as _,
    ) };
    unsafe { encore::items::init_allocator() };
}
