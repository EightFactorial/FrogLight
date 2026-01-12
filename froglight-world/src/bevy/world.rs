//! TODO

use core::ops::Deref;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    lifecycle::HookContext,
    reflect::ReflectComponent,
    world::{DeferredWorld, World},
};
use bevy_reflect::{Reflect, TypePath};
use froglight_common::{
    entity::{EntityId, EntityUuid},
    prelude::Identifier,
};
use hashbrown::HashMap;

use crate::{
    bevy::{ChunkOf, EntityOf, WorldChunks, WorldEntities},
    prelude::ChunkPos,
};

/// A world instance containing information about chunks and entities.
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(opaque, Debug, Clone, PartialEq, Component)]
#[require(WorldEntities, WorldChunks)]
pub struct WorldInstance {
    identifier: Identifier<'static>,
    chunk: HashMap<ChunkPos, Entity>,
    entity_id: HashMap<EntityId, Entity>,
    entity_uuid: HashMap<EntityUuid, Entity>,
}

impl WorldInstance {
    /// Create a new, empty [`WorldInstance`].
    #[must_use]
    pub fn new(identifier: Identifier<'static>) -> Self {
        Self {
            identifier,
            chunk: HashMap::new(),
            entity_id: HashMap::new(),
            entity_uuid: HashMap::new(),
        }
    }

    /// Get the identifier of this [`WorldInstance`].
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Query the [`WorldInstance`] for the [`Entity`] associated with the given
    /// data.
    ///
    /// Returns `None` if no such entity exists.
    #[must_use]
    #[expect(private_bounds, reason = "Only three possible data types")]
    pub fn get<T: InstanceData>(&self, data: &T) -> Option<Entity> { data.query(self) }
}

/// Assign `on_add` and `on_remove` hooks for components tracked by a
/// [`WorldInstance`].
///
/// # Panics
///
/// Panics if any of the components already have hooks assigned.
pub fn register_component_hooks(world: &mut World) {
    world
        .register_component_hooks::<ChunkPos>()
        .on_add(instance_add_hook::<ChunkPos>)
        .on_remove(instance_remove_hook::<ChunkPos>);
    world
        .register_component_hooks::<EntityId>()
        .on_add(instance_add_hook::<EntityId>)
        .on_remove(instance_remove_hook::<EntityId>);
    world
        .register_component_hooks::<EntityUuid>()
        .on_add(instance_add_hook::<EntityUuid>)
        .on_remove(instance_remove_hook::<EntityUuid>);
}

/// Hook for when an instance-tracked component is added to an entity.
fn instance_add_hook<T: InstanceData>(mut world: DeferredWorld, ctx: HookContext) {
    // Get the relationship component that points to the `WorldInstance`
    let Some(instance) = world.get::<T::Relationship>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {} added a `{}`, but has no `{}` component!", ctx.entity, T::short_type_path(), T::Relationship::short_type_path());
        return;
    };

    // Get both the trigger entity and the instance entity
    let Ok([trigger, mut instance]) = world.get_entity_mut([ctx.entity, **instance]) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `{}` points to a non-existent entity!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Get the trigger component and the instance component
    let trigger = trigger.get::<T>().expect("Component does not exist after being added?");
    let Some(mut instance) = instance.get_mut::<WorldInstance>() else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `{}` points to an entity that is not a `WorldInstance`!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Insert the new entity and despawn any previous entity
    if let Some(previous) = T::insert(trigger, &mut instance, ctx.entity) {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_world", "Entity {} is replacing Entity {} via `{}`, despawning!", ctx.entity, previous, T::short_type_path());
        world.commands().entity(previous).despawn();
    }
}

/// Hook for when an instance-tracked component is removed from an entity.
fn instance_remove_hook<T: InstanceData>(mut world: DeferredWorld, ctx: HookContext) {
    // Get the relationship component that points to the `WorldInstance`
    let Some(instance) = world.get::<T::Relationship>(ctx.entity) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {} removed a `{}`, but has no `{}` component!", ctx.entity, T::short_type_path(), T::Relationship::short_type_path());
        return;
    };

    // Get both the trigger entity and the instance entity
    let Ok([trigger, mut instance]) = world.get_entity_mut([ctx.entity, **instance]) else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `{}` points to a non-existent entity!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Get the instance component
    let Some(mut instance) = instance.get_mut::<WorldInstance>() else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `{}` points to an entity that is not a `WorldInstance`!", ctx.entity, T::Relationship::short_type_path());
        return;
    };

    // Remove the entity from the instance
    let removed = trigger.get::<T>().expect("Component does not exist after being removed?");
    if !T::remove(removed, &mut instance) {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_world", "Entity {}'s `{}` was not found in the `WorldInstance`!", ctx.entity, T::short_type_path());
    }
}

// -------------------------------------------------------------------------------------------------

trait InstanceData: Component + TypePath {
    /// The relationship component that points to the [`WorldInstance`].
    type Relationship: Component + TypePath + Deref<Target = Entity>;
    /// Query the [`WorldInstance`] for the associated [`Entity`].
    fn query(&self, instance: &WorldInstance) -> Option<Entity>;
    /// Insert the associated [`Entity`] into the [`WorldInstance`].
    ///
    /// Returns the previous entity if one existed.
    fn insert(&self, instance: &mut WorldInstance, entity: Entity) -> Option<Entity>;
    /// Remove the associated [`Entity`] from the [`WorldInstance`].
    ///
    /// Returns `true` if an entity was removed.
    fn remove(&self, instance: &mut WorldInstance) -> bool;
}

impl InstanceData for ChunkPos {
    type Relationship = ChunkOf;

    #[inline]
    fn query(&self, instance: &WorldInstance) -> Option<Entity> {
        instance.chunk.get(self).copied()
    }

    #[inline]
    fn insert(&self, instance: &mut WorldInstance, entity: Entity) -> Option<Entity> {
        instance.chunk.insert(*self, entity)
    }

    #[inline]
    fn remove(&self, instance: &mut WorldInstance) -> bool { instance.chunk.remove(self).is_some() }
}

impl InstanceData for EntityId {
    type Relationship = EntityOf;

    #[inline]
    fn query(&self, instance: &WorldInstance) -> Option<Entity> {
        instance.entity_id.get(self).copied()
    }

    #[inline]
    fn insert(&self, instance: &mut WorldInstance, entity: Entity) -> Option<Entity> {
        instance.entity_id.insert(*self, entity)
    }

    #[inline]
    fn remove(&self, instance: &mut WorldInstance) -> bool {
        instance.entity_id.remove(self).is_some()
    }
}

impl InstanceData for EntityUuid {
    type Relationship = EntityOf;

    #[inline]
    fn query(&self, instance: &WorldInstance) -> Option<Entity> {
        instance.entity_uuid.get(self).copied()
    }

    #[inline]
    fn insert(&self, instance: &mut WorldInstance, entity: Entity) -> Option<Entity> {
        instance.entity_uuid.insert(*self, entity)
    }

    #[inline]
    fn remove(&self, instance: &mut WorldInstance) -> bool {
        instance.entity_uuid.remove(self).is_some()
    }
}
