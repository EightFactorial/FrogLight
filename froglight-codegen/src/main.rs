#![doc = include_str!("../README.md")]
#![allow(clippy::alloc_instead_of_core, clippy::std_instead_of_core, reason = "Requires `std`")]
#![allow(unreachable_pub, reason = "Binary")]
#![allow(clippy::unnecessary_wraps, clippy::unused_async, reason = "WIP")]
#![allow(dead_code, unused_imports, reason = "WIP")]

use miette::Result;

mod common;
mod source;

fn main() -> Result<()> { Ok(()) }
