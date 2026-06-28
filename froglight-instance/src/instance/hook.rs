use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};
#[allow(unused_imports, reason = "Used by tracing macros")]
use bevy_reflect::TypePath;

use crate::{
    instance::{SessionInstance, data::InstanceData},
    relationship::PartOfInstance,
};

/// Hook for when an instance-tracked component is inserted into an entity.
pub(crate) fn insert_hook<T: InstanceData + Clone>(mut world: DeferredWorld, ctx: HookContext) {
    // Get the `PartOfInstance` component.
    let Some(instance) = world.get::<PartOfInstance>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_instance", "Entity {} added a `{}`, but has no `PartOfInstance` component!", ctx.entity, T::short_type_path());
        return;
    };

    // Get the trigger component.
    let trigger =
        world.get::<T>(ctx.entity).expect("Component does not exist after being added?").clone();

    // Get the instance component.
    let Some(mut instance) = world.get_mut::<SessionInstance>(instance.instance()) else {
        #[cfg(feature = "tracing")]
        tracing::error!(target: "froglight_instance", "Entity {} has an invalid `PartOfInstance` component!", ctx.entity);
        return;
    };

    // Insert the new entity and despawn any previous entity.
    if let Some(previous) = T::insert(trigger, &mut instance, ctx.entity)
        && previous != ctx.entity
    {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_instance", "Entity {} is replacing Entity {} via `{}`, despawning!", ctx.entity, previous, T::short_type_path());

        world.commands().entity(previous).despawn();
    }
}

/// Hook for when an instance-tracked component is removed from an entity.
pub(crate) fn discard_hook<T: InstanceData + Clone>(mut world: DeferredWorld, ctx: HookContext) {
    // Get the `PartOfInstance` component.
    let Some(instance) = world.get::<PartOfInstance>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_instance", "Entity {} removed a `{}`, but has no `PartOfInstance` component!", ctx.entity, T::short_type_path());
        return;
    };

    // Get the trigger component.
    let trigger =
        world.get::<T>(ctx.entity).expect("Component does not exist after being added?").clone();

    // Get the instance component.
    let Some(mut instance) = world.get_mut::<SessionInstance>(instance.instance()) else {
        #[cfg(feature = "tracing")]
        tracing::error!(target: "froglight_instance", "Entity {} has an invalid `PartOfInstance` component!", ctx.entity);
        return;
    };

    // Remove the entity from the instance.
    if !T::remove(&trigger, &mut instance) {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_instance", "Entity {}'s `{}` was not found in the `SessionInstance`!", ctx.entity, T::short_type_path());
    }
}
