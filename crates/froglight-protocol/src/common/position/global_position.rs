use froglight_macros::FrogReadWrite;

use super::BlockPosition;
use crate::common::ResourceKey;

/// A position in a world, measured in blocks.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct GlobalPosition {
    /// The world the position is in.
    pub world: ResourceKey,
    /// The position in the world.
    pub position: BlockPosition,
}

impl Default for GlobalPosition {
    fn default() -> Self {
        Self {
            world: ResourceKey::const_new("minecraft:overworld"),
            position: BlockPosition::default(),
        }
    }
}
