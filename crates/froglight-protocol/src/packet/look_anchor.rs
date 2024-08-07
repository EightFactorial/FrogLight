use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

/// An entity to look at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [24, 1])]
pub struct LookEntity {
    /// The entity to look at.
    pub entity: EntityId,
    /// The part of the entity to look at.
    pub anchor: LookAnchor,
}

/// The part of the entity to look at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0])]
pub enum LookAnchor {
    /// The entity's feet.
    Feet,
    /// The entity's eyes.
    Eyes,
}
