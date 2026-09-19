//! TODO

use core::any::TypeId;

use bevy_ecs::{
    component::Component,
    entity::{Entity, EntityHashSet, hash_set::Iter},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use froglight_biome::{storage::BiomeStorage, version::BiomeVersion};
use froglight_block::{storage::BlockStorage, version::BlockVersion};
use froglight_common::{crates::foldhash::fast::RandomState, prelude::Identifier, types::HashMap};
use froglight_entity::{
    prelude::{EntityId, EntityUuid},
    storage::EntityStorage,
    version::EntityVersion,
};
use froglight_item::{storage::ItemStorage, version::ItemVersion};
use froglight_world::{
    component::{BlockPos, ChunkBlockPos},
    prelude::ChunkPos,
};

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
    entity_id: HashMap<EntityId, Entity>,
    entity_uuid: HashMap<EntityUuid, Entity>,
    chunk_pos: HashMap<ChunkPos, Entity>,
}

impl SessionInstance {
    /// Create a new, empty [`SessionInstance`].
    #[must_use]
    pub fn new<V: BiomeVersion + BlockVersion + EntityVersion + ItemVersion>(
        dimension: Identifier<'static>,
        height_max: u32,
        height_min: i32,
    ) -> Self {
        Self {
            dimension,
            height_max_min: (height_max, height_min),

            v_biomes: V::biomes(),
            v_blocks: V::blocks(),
            v_entities: V::entities(),
            v_items: V::items(),

            entity: EntityHashSet::new(),
            entity_id: HashMap::with_hasher(RandomState::default()),
            entity_uuid: HashMap::with_hasher(RandomState::default()),
            chunk_pos: HashMap::with_hasher(RandomState::default()),
        }
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

    /// Convert a [`BlockPos`] into a [`ChunkBlockPos`] if it is within bounds.
    ///
    /// Moves from "world-space" to "chunk-space", where `0` is always the
    /// bottom of the chunk.
    #[inline]
    #[must_use]
    pub const fn get_chunkpos(&self, position: BlockPos) -> Option<ChunkBlockPos> {
        ChunkBlockPos::try_from_blockpos(position, self.height_min())
    }

    /// Get the [`BiomeStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn biomes(&self) -> &'static BiomeStorage { self.v_biomes }

    /// Get the [`BlockStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn blocks(&self) -> &'static BlockStorage { self.v_blocks }

    /// Get the [`EntityStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn entities(&self) -> &'static EntityStorage { self.v_entities }

    /// Get the [`ItemStorage`] for this [`SessionInstance`].
    #[inline]
    #[must_use]
    pub const fn items(&self) -> &'static ItemStorage { self.v_items }

    /// Returns `true` if the [`SessionInstance`] is of the given
    /// [`Version`](froglight_common::version::Version) type.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.is_version_ty(TypeId::of::<V>()) }

    /// Returns `true` if the [`SessionInstance`] is of the given
    /// [`Version`](froglight_common::version::Version) type.
    #[inline]
    #[must_use]
    pub fn is_version_ty(&self, ty: TypeId) -> bool { self.v_biomes.version_ty() == ty }

    /// Get a reference to the [`EntityHashSet`] of owned [`Entities`](Entity).
    #[inline]
    #[must_use]
    pub const fn entity_set(&self) -> &EntityHashSet { &self.entity }

    /// Get an iterator over all [`Entities`](Entity) in the
    /// [`SessionInstance`].
    #[inline]
    #[must_use]
    pub fn iter_entity(&self) -> Iter<'_, Entity> { self.entity.iter() }
}
