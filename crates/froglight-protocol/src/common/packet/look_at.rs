use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

/// The part of the entity to look at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum LookAnchor {
    /// The entity's feet.
    Feet,
    /// The entity's eyes.
    Eyes,
}

/// An entity to look at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [24, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct LookEntity {
    /// The entity to look at.
    pub entity: EntityId,
    /// The part of the entity to look at.
    pub anchor: LookAnchor,
}
