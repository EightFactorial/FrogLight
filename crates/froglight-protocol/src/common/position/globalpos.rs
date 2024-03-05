use super::BlockPosition;
use crate::common::ResourceKey;

/// A position in a world, measured in blocks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// TODO: #[frog(tests = ["read_example"], bytes = [19, 109, 105, 110, 101, 99,
// 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 0, 0, 0,
// 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct GlobalPosition {
    /// The world the position is in.
    pub world: ResourceKey,
    /// The position in the world.
    pub position: BlockPosition,
}

impl Default for GlobalPosition {
    fn default() -> Self {
        Self {
            world: ResourceKey::new_inline("minecraft:overworld"),
            position: BlockPosition::default(),
        }
    }
}
