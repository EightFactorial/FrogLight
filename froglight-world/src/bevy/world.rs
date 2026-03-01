//! TODO

use bevy_ecs::{
    component::Component, entity::Entity, lifecycle::HookContext, reflect::ReflectComponent,
    world::DeferredWorld,
};
use bevy_reflect::Reflect;
use hashbrown::HashMap;

use crate::prelude::ChunkPos;

/// A world instance containing information about chunks.
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(opaque, Debug, Clone, PartialEq, Component)]
#[component(on_remove = WorldInstanceChunks::remove_hook)]
pub struct WorldInstanceChunks {
    height_max: u32,
    height_min: i32,
    pub(super) chunks: HashMap<ChunkPos, Entity>,
}

impl WorldInstanceChunks {
    /// Create a new, empty [`WorldInstanceChunks`].
    ///
    /// # Panics
    ///
    /// Panics if the maximum height is less than the minimum height.
    #[must_use]
    pub fn new(height_max: u32, height_min: i32) -> Self {
        assert!(
            height_min.is_negative() || height_max >= height_min.unsigned_abs(),
            "Maximum height must be greater than or equal to minimum height!"
        );

        Self { chunks: HashMap::new(), height_max, height_min }
    }

    /// Get the maximum height of the world.
    #[inline]
    #[must_use]
    pub const fn height_max(&self) -> u32 { self.height_max }

    /// Get the minimum height of the world.
    #[inline]
    #[must_use]
    pub const fn height_min(&self) -> i32 { self.height_min }

    /// Get the total height of the world.
    ///
    /// # Panics
    ///
    /// Panics if the maximum height is less than the minimum height.
    #[inline]
    #[must_use]
    pub const fn height_total(&self) -> u32 { self.height_max.strict_sub_signed(self.height_min) }

    /// Query the [`WorldInstanceChunks`] for the [`Entity`] associated with the
    /// given [`ChunkPos`].
    ///
    /// Returns `None` if no such entity exists.
    #[must_use]
    pub fn get(&self, position: &ChunkPos) -> Option<Entity> { self.chunks.get(position).copied() }

    /// Get an iterator over the chunks in this [`WorldInstanceChunks`].
    pub fn iter(&self) -> impl Iterator<Item = (&ChunkPos, &Entity)> { self.chunks.iter() }

    #[allow(unused_variables, reason = "Used by tracing macros")]
    fn remove_hook(mut world: DeferredWorld, ctx: HookContext) {
        let mut instance = world
            .get_mut::<WorldInstanceChunks>(ctx.entity)
            .expect("WorldInstanceChunks does not exist after being removed?");

        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_world", "Removing WorldInstanceChunks from Entity {}!", ctx.entity);
        let mut chunk_map = core::mem::take(&mut instance.chunks);

        for (chunk_pos, entity) in chunk_map.drain() {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_world", "Despawning Entity {} associated with EntityId {:?}!", entity, chunk_pos);
            world.commands().entity(entity).despawn();
        }
    }
}
