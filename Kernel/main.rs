/*
 * Rust BareBones OS
 * - By John Hodge (Mutabah/thePowersGang) 
 *
 * main.rs
 * - Top-level file for kernel
 *
 * This code has been put into the public domain, there are no restrictions on
 * its use, and the author takes no liability.
 */
#![no_std]	//< Kernels can't use std
#![feature(lang_items)]	//< unwind needs to define lang items
#![feature(asm)]	//< As a kernel, we need inline assembly
#![feature(core)]	//< libcore (see below) is not yet stablized

use prelude::*;

// Load libcore (it's nice and freestanding)
// - We want the macros from libcore
#[macro_use]
extern crate core;

/// A dummy 'std' module to work around a set of issues in rustc
mod std {
	// #18491 - write!() expands to std::fmt::Arguments::new
	pub use core::fmt;
}

/// Macros, need to be loaded before everything else due to how rust parses
#[macro_use]
mod macros;

// Achitecture-specific modules
#[cfg(arch__amd64)] #[path="arch/amd64/mod.rs"]
pub mod arch;
#[cfg(arch__x86)] #[path="arch/x86/mod.rs"]
pub mod arch;

// Prelude
mod prelude;

/// Exception handling (panic)
pub mod unwind;

/// Logging code
mod logging;

// Kernel entrypoint
#[lang="start"]
#[no_mangle]
fn kmain()
{
	log!("Hello world!");
	loop {}
}

