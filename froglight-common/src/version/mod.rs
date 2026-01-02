//! The [`Version`] trait and generated versions.

use core::{fmt::Debug, hash::Hash};

mod generated;
pub use generated::*;

/// A version.
pub trait Version: Debug + Default + Copy + Eq + Hash + Send + Sync + 'static {
    /// The protocol id.
    const PROTOCOL_ID: u32;
    /// The resource pack version.
    const RESOURCE_VERSION: u32;
}
