//! TODO

use alloc::string::String;

/// A known resource pack.
///
/// Used to identify a resource pack we already know about.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct KnownResourcePack {
    /// The namespace of the resource pack
    pub namespace: String,
    /// The id of the resource pack
    pub pack_id: String,
    /// The version of the resource pack
    pub version: String,
}
