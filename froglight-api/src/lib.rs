#![doc = include_str!("../README.md")]
#![allow(clippy::std_instead_of_alloc, reason = "Requires the standard library")]
#![allow(clippy::std_instead_of_core, reason = "Requires the standard library")]

pub mod agent;
#[cfg(feature = "bevy")]
pub mod bevy;
pub mod resolver;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{agent::HttpAgent, resolver::DnsResolver};
}
