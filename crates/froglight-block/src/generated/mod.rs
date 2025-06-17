//! TODO
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt::skip)]

pub mod attribute;
pub mod block;

#[cfg(feature = "v1_21_4")]
pub mod v1_21_4;
#[cfg(feature = "v1_21_5")]
pub mod v1_21_5;
#[cfg(feature = "v1_21_6")]
pub mod v1_21_6;
