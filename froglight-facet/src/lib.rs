#![doc = include_str!("../README.md")]
#![allow(unused_features, reason = "Enabled features may not be used on all platform")]
#![cfg_attr(feature = "nightly", feature(const_array, const_trait_impl, portable_simd))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
#[cfg(feature = "simd")]
pub mod simd;

pub mod facet;

pub mod deserialize;
pub use deserialize::functions::*;

pub mod serialize;
pub use serialize::functions::*;
