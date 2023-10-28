#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(int_roundings)]
#![feature(cursor_remaining)]

pub mod buffer;
pub mod types;
pub mod versions;

mod connection;
pub use connection::*;

mod traits;
pub use traits::*;

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
