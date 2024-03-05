use froglight_macros::FrogReadWrite;

/// The player's hand.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FrogReadWrite)]
// TODO: #[frog(tests = ["read_default", "write_example"], bytes = [1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum PlayerHand {
    /// The player's left hand
    Left,
    /// The player's right hand
    #[default]
    Right,
}
