use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::{
    bevy::world::WorldInstanceChunks,
    prelude::{ChunkOfInstance, ChunkPos},
};

/// Hook for when an instance-tracked component is added to an entity.
pub(super) fn instance_add_hook(mut world: DeferredWorld, ctx: HookContext) {
    // Get the relationship component that points to the `WorldInstanceChunks`
    let Some(instance) = world.get::<ChunkOfInstance>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {} added a `ChunkPos`, but has no `ChunkOfInstance` component!", ctx.entity);
        return;
    };

    // Get both the trigger entity and the instance entity
    let Ok([trigger, mut instance]) = world.get_entity_mut([ctx.entity, **instance]) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `ChunkOfInstance` points to a non-existent entity!", ctx.entity);
        return;
    };

    // Get the trigger component and the instance component
    let trigger = trigger.get::<ChunkPos>().expect("Component does not exist after being added?");
    let Some(mut instance) = instance.get_mut::<WorldInstanceChunks>() else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `ChunkOfInstance` points to an entity that is not a `WorldInstanceChunks`!", ctx.entity);
        return;
    };

    // Insert the new entity and despawn any previous entity
    if let Some(previous) = instance.chunks.insert(*trigger, ctx.entity) {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_world", "Entity {} is replacing Entity {previous} via `ChunkPos`, despawning!", ctx.entity);
        world.commands().entity(previous).despawn();
    }
}

/// Hook for when an instance-tracked component is removed from an entity.
pub(super) fn instance_remove_hook(mut world: DeferredWorld, ctx: HookContext) {
    // Get the relationship component that points to the `WorldInstanceChunks`
    let Some(instance) = world.get::<ChunkOfInstance>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {} removed a `ChunkPos`, but has no `ChunkOfInstance` component!", ctx.entity);
        return;
    };

    // Get both the trigger entity and the instance entity
    let Ok([trigger, mut instance]) = world.get_entity_mut([ctx.entity, **instance]) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `ChunkOfInstance` points to a non-existent entity!", ctx.entity);
        return;
    };

    // Get the instance component
    let Some(mut instance) = instance.get_mut::<WorldInstanceChunks>() else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `ChunkOfInstance` points to an entity that is not a `WorldInstanceChunks`!", ctx.entity);
        return;
    };

    // Remove the entity from the instance
    let removed = trigger.get::<ChunkPos>().expect("Component does not exist after being removed?");
    if instance.chunks.remove(removed).is_none() {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `ChunkPos` was not found in the `WorldInstanceChunks`!", ctx.entity);
    }
}
