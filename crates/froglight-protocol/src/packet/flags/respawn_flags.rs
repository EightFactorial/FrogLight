use froglight_macros::FrogReadWrite;

/// Flags for the player respawn packet.
///
/// Examples:
/// - Death: `{ false, false }`
/// - Credits: `{ true, false }`
/// - Change Dimension: `{ true, true }`
// TODO: Document this struct
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(bitset, tests = ["read_verify", "write_verify"], bytes = [0])]
pub struct PlayerRespawnFlags {
    /// Keep Attributes
    pub keep_attributes: bool,
    /// Keep Metadata
    pub keep_metadata: bool,
}
