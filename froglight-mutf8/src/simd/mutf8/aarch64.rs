//! TODO
#![allow(clippy::wildcard_imports, reason = "`aarch64` module")]

#[expect(unused_imports, reason = "WIP")]
use core::{arch::aarch64::*, simd::prelude::*};

pub use super::fallback::{contains_4_byte_header, contains_null_or_4_byte_header, utf8_to_mutf8};
