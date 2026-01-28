//! The [`Version`] trait and generated versions.

use core::{fmt::Debug, hash::Hash};

mod generated;
#[allow(unused_imports, unreachable_pub, reason = "Triggered if no versions are enabled")]
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
