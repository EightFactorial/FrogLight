use froglight_macros::FrogReadWrite;

/// Flags for player input.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(bitset, tests = ["read_verify", "write_verify"], bytes = [0])]
pub struct PlayerInputFlags {
    /// Whether the player is currently jumping.
    pub jumping: bool,
    /// Whether the player is currently sneaking.
    pub shift: bool,
}
