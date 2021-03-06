/*
 * Rust BareBones OS
 * - By John Hodge (Mutabah/thePowersGang) 
 *
 * logging.rs
 * - Debug output using rust's core::fmt system
 *
 * This code has been put into the public domain, there are no restrictions on
 * its use, and the author takes no liability.
 */
use prelude::*;
use core::atomic;
use core::fmt;

/// A formatter object
pub struct Writer(bool);

/// A primitive lock for the logging output
static LOGGING_LOCK: atomic::AtomicBool = atomic::ATOMIC_BOOL_INIT;

impl Writer
{
	/// Obtain a logger for the specified module
	pub fn get(module: &str) -> Writer {
		// This "acquires" a lock (actually just disables output if paralel writes are attempted
		let mut ret = Writer( ! LOGGING_LOCK.swap(true, atomic::Ordering::Acquire) );
		
		// Print the module name before returning (prefixes all messages)
		{
			use core::fmt::Writer;
			write!(&mut ret, "[{}] ", module);
		}
		
		ret
	}
}

impl ::core::ops::Drop for Writer
{
	fn drop(&mut self)
	{
		// Write a terminating newline before releasing the lock
		{
			use core::fmt::Writer;
			write!(self, "\n");
		}
		// On drop, "release" the lock
		if self.0 {
			LOGGING_LOCK.store(false, atomic::Ordering::Release);
		}
	}
}

impl fmt::Writer for Writer
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		// If the lock is owned by this instance, then we can safely write to the output
		if self.0
		{
			unsafe {
				::arch::debug::puts( s );
			}
		}
		Ok( () )
	}
}

