//! The [`Version`] trait and generated versions.

use core::{fmt::Debug, hash::Hash};

/// A trait representing a version.
pub trait Version: Debug + Copy + Eq + Hash + Send + Sync + 'static {
    /// The data version.
    ///
    /// Used in determine compatibility with world data.
    const DATA: u32;
    /// The protocol version.
    ///
    /// Used to determine compatibility between clients and servers.
    const PROTOCOL: u32;
    /// The data pack version.
    ///
    /// Used to determine compatibility with data packs.
    const DATA_PACK: u32;
    /// The resource pack version.
    ///
    /// Used to determine compatibility with resource packs.
    const RESOURCE_PACK: u32;
}

// -------------------------------------------------------------------------------------------------
// Note: The following versions are automatically @generated.

/// Minecraft v1.21.8
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct V1_21_8;

impl Version for V1_21_8 {
    const DATA: u32 = 4440;
    const DATA_PACK: u32 = 81;
    const PROTOCOL: u32 = 772;
    const RESOURCE_PACK: u32 = 64;
}
