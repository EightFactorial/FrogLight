//! TODO
#![allow(clippy::all, clippy::unreadable_literal)]
#![cfg_attr(rustfmt, rustfmt::skip)]

mod attribute;
pub use attribute::*;

#[cfg(feature = "v1_21_4")]
pub mod v1_21_4;
#[cfg(feature = "v1_21_5")]
pub mod v1_21_5;
