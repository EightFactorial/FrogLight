use froglight_macros::FrogReadWrite;

/// Model customization options for a player's skin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(bitset, tests = ["read_example"], bytes = [0])]
pub struct PlayerModelFlags {
    /// The player's cape.
    pub cape: bool,
    /// The player's jacket.
    pub jacket: bool,
    /// The player's left sleeve.
    pub left_sleeve: bool,
    /// The player's right sleeve.
    pub right_sleeve: bool,
    /// The player's left pants.
    pub left_pants: bool,
    /// The player's right pants.
    pub right_pants: bool,
    /// The player's hat.
    pub hat: bool,
}

impl Default for PlayerModelFlags {
    fn default() -> Self {
        Self {
            cape: true,
            jacket: true,
            left_sleeve: true,
            right_sleeve: true,
            left_pants: true,
            right_pants: true,
            hat: true,
        }
    }
}
