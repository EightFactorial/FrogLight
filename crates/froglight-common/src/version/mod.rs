//! The [`Version`] trait and generated versions.

use std::{fmt::Debug, hash::Hash};

mod generated;
#[allow(unreachable_pub, unused_imports)]
pub use generated::*;

/// A version.
pub trait Version: Debug + Default + Copy + Eq + Hash + Send + Sync + 'static {
    /// The protocol id.
    const PROTOCOL_ID: u32;
    /// The resource pack version.
    const RESOURCE_VERSION: u32;
}
