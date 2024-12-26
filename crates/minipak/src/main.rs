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

use encore::prelude::*;

#[unsafe(no_mangle)]
unsafe fn pre_main(_stack_top: *mut u8) {
    unsafe {
        init_allocator();
        main().unwrap();
        syscall::exit(0);
    }
}

fn main() -> Result<(), EncoreError> {
    let file = File::open("/lib64/ld-linux-x86-64.so.2")?;
    let map = file.map()?;

    let there_you_go = core::str::from_utf8(&map[1..4]).unwrap();
    println!("{}", there_you_go);

    Ok(())
}
