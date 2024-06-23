use froglight_macros::FrogReadWrite;

/// An entity animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [4])]
pub enum EntityAnimation {
    /// The entity's swings its main hand.
    SwingMainHand,
    /// The entity's is hurt.
    Hurt,
    /// The entity's wakes up.
    WakeUp,
    /// The entity's swings its off hand.
    SwingOffHand,
    /// The entity's performs a critical hit.
    CriticalHit,
    /// The entity's performs a magic critical hit.
    MagicCriticalHit,
}
