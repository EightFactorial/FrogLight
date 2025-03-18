use froglight_nbt::nbt::UnnamedNbt;

use crate::position::RelativeBlockPos;

/// A block entity in a [`Chunk`](crate::chunk::Chunk).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "io", derive(froglight_io::prelude::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PackedEntity {
    /// The entity's chunk-relative position.
    pub position: RelativeBlockPos,
    /// The entity's type.
    #[cfg_attr(feature = "io", frog(var))]
    pub entity_type: u32,
    /// The entity's NBT data.
    pub entity_data: UnnamedNbt,
}

impl PackedEntity {
    /// A helper function to convert a list of entities into a position map.
    #[must_use]
    pub fn list_into_map(entities: Vec<Self>) -> hashbrown::HashMap<RelativeBlockPos, Self> {
        entities.into_iter().map(|entity| (entity.position, entity)).collect()
    }
}
