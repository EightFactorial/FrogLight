#![doc = include_str!("../README.md")]
#![cfg_attr(any(docsrs, docsrs_dep), allow(internal_features, reason = "rustdoc_internals"))]
#![cfg_attr(any(docsrs, docsrs_dep), feature(rustdoc_internals))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod bevy;
pub mod builder;
pub mod graph;
pub mod traits;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{graph::CommandGraph, traits::AddGameCommand};
}
