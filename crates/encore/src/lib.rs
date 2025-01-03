// Don't use libstd
#![no_std]
// Allow specifying our own eh_personality
#![feature(lang_items)]
// Allow using intrinsics
#![feature(core_intrinsics)]

// Bring in heap-allocated types like `Vec`, and `String`
extern crate alloc;

pub mod error; // new!
pub mod fs;
pub mod items;
pub mod memmap; // also new!
pub mod prelude;
pub mod syscall;
pub mod utils;
pub mod env;