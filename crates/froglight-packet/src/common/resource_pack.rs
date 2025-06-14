#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use smol_str::SmolStr;

/// A known resource pack.
///
/// Used to verify a client and server are using the same resource packs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub struct KnownResourcePack {
    /// The namespace of the resource pack.
    pub namespace: SmolStr,
    /// The identifier of the resource pack.
    pub identifier: SmolStr,
    /// The version of the resource pack.
    pub version: SmolStr,
}

// -------------------------------------------------------------------------------------------------

/// The client's status of downloading and applying a resource pack.
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum ResourcePackStatus {
    SuccessfullyLoaded,
    Declined,
    DownloadFailed,
    Accepted,
    InvalidUrl,
    ReloadFailed,
    Discarded,
}
