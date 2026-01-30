//! The [`Version`] trait and generated versions.
#![allow(clippy::unreadable_literal, reason = "Generated code")]
#![allow(unused_imports, unreachable_pub, reason = "Triggered if no features are enabled")]

use core::{fmt::Debug, hash::Hash};

mod generated;
pub use generated::*;

/// A version.
pub trait Version: Debug + Default + Copy + Eq + Hash + Send + Sync + 'static {
    /// The world data version.
    const DATA_VERSION: u32;
    /// The protocol id.
    const PROTOCOL_ID: u32;
    /// The resource pack version.
    const RESOURCE_VERSION: u32;
}
