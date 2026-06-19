#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod deserialize;
pub mod serialize;

mod reader;
pub use reader::{Reader, ReaderError};

mod writer;
pub use writer::{Writer, WriterError, WriterType};
