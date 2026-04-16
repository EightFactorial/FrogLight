#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(const_array, const_trait_impl, portable_simd))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "jit")]
pub mod jit;
#[cfg(feature = "simd")]
pub mod simd;

pub mod facet;
