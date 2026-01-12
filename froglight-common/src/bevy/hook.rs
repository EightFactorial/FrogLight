use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};
#[allow(unused_imports, reason = "Used by tracing macros")]
use bevy_reflect::TypePath;

use crate::{bevy::world::InstanceData, prelude::WorldInstance};

/// Hook for when an instance-tracked component is added to an entity.
pub(super) fn instance_add_hook<T: InstanceData>(mut world: DeferredWorld, ctx: HookContext) {
    // Get the relationship component that points to the `WorldInstance`
    let Some(instance) = world.get::<T::Relationship>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_common", "Entity {} added a `{}`, but has no `{}` component!", ctx.entity, T::short_type_path(), T::Relationship::short_type_path());
        return;
    };

    // Get both the trigger entity and the instance entity
    let Ok([trigger, mut instance]) = world.get_entity_mut([ctx.entity, **instance]) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_common", "Entity {}'s `{}` points to a non-existent entity!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Get the trigger component and the instance component
    let trigger = trigger.get::<T>().expect("Component does not exist after being added?");
    let Some(mut instance) = instance.get_mut::<WorldInstance>() else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_common", "Entity {}'s `{}` points to an entity that is not a `WorldInstance`!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Insert the new entity and despawn any previous entity
    if let Some(previous) = T::insert(trigger, &mut instance, ctx.entity) {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_common", "Entity {} is replacing Entity {} via `{}`, despawning!", ctx.entity, previous, T::short_type_path());
        world.commands().entity(previous).despawn();
    }
}

/// Hook for when an instance-tracked component is removed from an entity.
pub(super) fn instance_remove_hook<T: InstanceData>(mut world: DeferredWorld, ctx: HookContext) {
    // Get the relationship component that points to the `WorldInstance`
    let Some(instance) = world.get::<T::Relationship>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_common", "Entity {} removed a `{}`, but has no `{}` component!", ctx.entity, T::short_type_path(), T::Relationship::short_type_path());
        return;
    };

    // Get both the trigger entity and the instance entity
    let Ok([trigger, mut instance]) = world.get_entity_mut([ctx.entity, **instance]) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_common", "Entity {}'s `{}` points to a non-existent entity!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Get the instance component
    let Some(mut instance) = instance.get_mut::<WorldInstance>() else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_common", "Entity {}'s `{}` points to an entity that is not a `WorldInstance`!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Remove the entity from the instance
    let removed = trigger.get::<T>().expect("Component does not exist after being removed?");
    if !T::remove(removed, &mut instance) {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_common", "Entity {}'s `{}` was not found in the `WorldInstance`!", ctx.entity, T::short_type_path());
    }
}
