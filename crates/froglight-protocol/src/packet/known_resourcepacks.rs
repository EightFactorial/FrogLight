use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

/// Information about a known resource pack.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct KnownResourcePack {
    /// The namespace of the resource pack.
    pub namespace: CompactString,
    /// The ID of the resource pack.
    pub id: CompactString,
    /// The version of the resource pack.
    pub version: CompactString,
}
