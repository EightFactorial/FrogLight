use froglight_macros::FrogReadWrite;

/// A list of known resource packs.
///
/// Used to inform the client/server of the resource packs that are available.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct KnownResourcePacks {
    /// The list of resource packs.
    pub resourcepacks: Vec<ResourcePackInfo>,
}

/// Information about a resource pack.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ResourcePackInfo {
    /// The namespace of the resource pack.
    pub namespace: String,
    /// The ID of the resource pack.
    pub id: String,
    /// The version of the resource pack.
    pub version: String,
}
