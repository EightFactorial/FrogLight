#![allow(clippy::inline_always, reason = "Performance")]
#![allow(clippy::many_single_char_names, reason = "Readability")]

pub mod contains;
pub use contains::{contains_4_byte_header, contains_null_or_4_byte_header};

#[cfg(feature = "alloc")]
pub mod reencode;
#[cfg(feature = "alloc")]
pub use reencode::{mutf8_to_utf8, utf8_to_mutf8};
