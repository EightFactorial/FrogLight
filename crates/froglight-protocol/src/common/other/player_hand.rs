#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
use froglight_macros::FrogReadWrite;

/// The player's hand.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [1])]
pub enum PlayerHand {
    /// The player's left hand
    Left,
    /// The player's right hand
    #[default]
    Right,
}

/// The hand the player is using to interact with
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum InteractionHand {
    /// The player's main hand
    #[default]
    MainHand,
    /// The player's off hand
    OffHand,
}
