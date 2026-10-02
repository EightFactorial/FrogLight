#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", allow(stable_features, reason = "Targets Rust 1.100"))]
#![cfg_attr(feature = "nightly", feature(exclusive_wrapper))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod cache;
pub mod deserialize;
pub mod serialize;
pub mod solver;

mod reader;
pub use reader::{Reader, ReaderError};

mod writer;
pub use writer::{Writer, WriterError, WriterType};
