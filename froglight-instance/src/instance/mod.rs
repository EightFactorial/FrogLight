//! TODO

use bevy_ecs::{
    component::Component,
    entity::{Entity, EntityHashSet, hash_set::Iter},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use foldhash::fast::FixedState;
use froglight_biome::{storage::BiomeStorage, version::BiomeVersion};
use froglight_block::{storage::BlockStorage, version::BlockVersion};
use froglight_common::prelude::Identifier;
use froglight_entity::{
    prelude::{EntityId, EntityUuid},
    storage::EntityStorage,
    version::EntityVersion,
};
use froglight_item::{storage::ItemStorage, version::ItemVersion};
use froglight_world::prelude::ChunkPos;
use hashbrown::HashMap;

pub(crate) mod data;
pub(crate) mod hook;
pub(crate) mod reflect;

/// An instance of a session.
///
/// Tracks information about which entities belong to the session and more.
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(opaque, Debug, Clone, Component)]
pub struct SessionInstance {
    dimension: Identifier<'static>,
    height_max_min: (u32, i32),

    v_biomes: &'static BiomeStorage,
    v_blocks: &'static BlockStorage,
    v_entities: &'static EntityStorage,
    v_items: &'static ItemStorage,

    entity: EntityHashSet,
    entity_id: HashMap<EntityId, Entity, FixedState>,
    entity_uuid: HashMap<EntityUuid, Entity, FixedState>,
    chunk_pos: HashMap<ChunkPos, Entity, FixedState>,
}

impl SessionInstance {
    /// Create a new, empty [`SessionInstance`].
    #[must_use]
    pub fn new<V: BiomeVersion + BlockVersion + EntityVersion + ItemVersion>(
        dimension: Identifier<'static>,
        height_max: u32,
        height_min: i32,
    ) -> Self {
        let bytes = dimension.as_str().as_bytes();
        let mut seed_a = Self::create_seed(0, bytes);
        let mut seed_b = Self::create_seed(2, bytes);
        let mut seed_c = Self::create_seed(4, bytes);

        // Fiddle with the seeds a bit.
        seed_a ^= seed_c.rotate_left(8);
        seed_b ^= seed_a.rotate_left(9);
        seed_c ^= seed_b.rotate_left(10);

        Self {
            dimension,
            height_max_min: (height_max, height_min),

            v_biomes: V::biomes(),
            v_blocks: V::blocks(),
            v_entities: V::entities(),
            v_items: V::items(),

            entity: EntityHashSet::new(),
            entity_id: HashMap::with_hasher(FixedState::with_seed(seed_a)),
            entity_uuid: HashMap::with_hasher(FixedState::with_seed(seed_b)),
            chunk_pos: HashMap::with_hasher(FixedState::with_seed(seed_c)),
        }
    }

    /// Create a [`u64`] seed from `bytes[index..index+7]`.
    #[must_use]
    const fn create_seed(index: usize, bytes: &[u8]) -> u64 {
        let mut array = [0u8; 8];
        let mut i = 0;

        while i < 8 && i + index < bytes.len() {
            array[i] = bytes[i + index];
            i += 1;
        }

        u64::from_le_bytes(array)
    }

    /// Get the dimension's [`Identifier`].
    #[inline]
    #[must_use]
    pub const fn dimension(&self) -> &Identifier<'static> { &self.dimension }

    /// Get the maximum height of the world.
    #[inline]
    #[must_use]
    pub const fn height_max(&self) -> u32 { self.height_max_min.0 }

    /// Get the minimum height of the world.
    #[inline]
    #[must_use]
    pub const fn height_min(&self) -> i32 { self.height_max_min.1 }

    /// Get the [`BiomeStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn version_biomes(&self) -> &'static BiomeStorage { self.v_biomes }

    /// Get the [`BlockStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn version_blocks(&self) -> &'static BlockStorage { self.v_blocks }

    /// Get the [`EntityStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn version_entities(&self) -> &'static EntityStorage { self.v_entities }

    /// Get the [`ItemStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn version_items(&self) -> &'static ItemStorage { self.v_items }

    /// Returns `true` if the given [`Entity`] is part of the instance.
    #[inline]
    #[must_use]
    pub fn contains_entity(&self, entity: &Entity) -> bool { self.entity.contains(entity) }

    /// Get a reference to the [`EntityHashSet`] of related entities.
    #[inline]
    #[must_use]
    pub const fn entity_map(&self) -> &EntityHashSet { &self.entity }

    /// Get an iterator over all [`Entity`]s in the [`SessionInstance`].
    #[inline]
    #[must_use]
    pub fn iter_entity(&self) -> Iter<'_> { self.entity.iter() }
}
