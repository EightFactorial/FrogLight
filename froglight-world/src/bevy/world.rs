//! TODO

use bevy_ecs::{
    component::Component, entity::Entity, lifecycle::HookContext, reflect::ReflectComponent,
    world::DeferredWorld,
};
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use hashbrown::HashMap;

use crate::prelude::ChunkPos;

/// A world instance containing information about chunks.
#[derive(Debug, Default, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(opaque, Debug, Default, Clone, PartialEq, Component)]
#[component(on_remove = WorldInstanceChunks::remove_hook)]
pub struct WorldInstanceChunks {
    pub(super) chunks: HashMap<ChunkPos, Entity>,
}

impl WorldInstanceChunks {
    /// Create a new, empty [`WorldInstanceChunks`].
    #[must_use]
    pub fn new() -> Self { Self { chunks: HashMap::new() } }

    /// Query the [`WorldInstanceChunks`] for the [`Entity`] associated with the
    /// given [`ChunkPos`].
    ///
    /// Returns `None` if no such entity exists.
    #[must_use]
    pub fn get(&self, position: &ChunkPos) -> Option<Entity> { self.chunks.get(position).copied() }

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
