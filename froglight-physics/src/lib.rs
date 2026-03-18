#![cfg_attr(feature = "nightly", feature(portable_simd), allow(unused_features, reason = "WIP"))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod components;
pub mod controller;
pub mod state;
pub mod step;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{components::*, controller::PhysicsController, state::PhysicsState};
}
