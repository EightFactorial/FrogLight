//! TODO

use core::ops::Deref;

use bevy_ecs::{
    component::Component, entity::Entity, lifecycle::HookContext, reflect::ReflectComponent,
    world::DeferredWorld,
};
use bevy_reflect::{Reflect, TypePath};
use hashbrown::HashMap;

use crate::{
    bevy::EntityOfInstance,
    entity::{EntityId, EntityUuid},
    prelude::Identifier,
};

/// A world instance containing information about entities.
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(opaque, Debug, Clone, PartialEq, Component)]
#[component(on_remove = WorldInstance::remove_hook)]
pub struct WorldInstance {
    identifier: Identifier<'static>,
    entity_id: HashMap<EntityId, Entity>,
    entity_uuid: HashMap<EntityUuid, Entity>,
}

impl WorldInstance {
    /// Create a new, empty [`WorldInstance`].
    #[must_use]
    pub fn new(identifier: Identifier<'static>) -> Self {
        Self { identifier, entity_id: HashMap::new(), entity_uuid: HashMap::new() }
    }

    /// Get the identifier of this [`WorldInstance`].
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Query the [`WorldInstance`] for the [`Entity`] associated with the given
    /// data.
    ///
    /// Returns `None` if no such entity exists.
    ///
    /// ## Note
    ///
    /// There are only two supported data types:
    ///   - [`EntityId`]
    ///   - [`EntityUuid`]
    #[must_use]
    #[expect(private_bounds, reason = "Only two possible data types")]
    pub fn get<T: InstanceData>(&self, data: &T) -> Option<Entity> { data.query(self) }

    /// Hook for when a [`WorldInstance`] is removed from an entity.
    #[allow(unused_variables, reason = "Used by tracing macros")]
    fn remove_hook(mut world: DeferredWorld, ctx: HookContext) {
        let mut instance = world
            .get_mut::<WorldInstance>(ctx.entity)
            .expect("WorldInstance does not exist after being removed?");

        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_common", "Removing WorldInstance \"{}\" from Entity {}!", instance.identifier(), ctx.entity);
        let (mut id_map, mut uuid_map) =
            (core::mem::take(&mut instance.entity_id), core::mem::take(&mut instance.entity_uuid));

        for (entity_id, entity) in id_map.drain() {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_common", "Despawning Entity {} associated with EntityId {:?}!", entity, entity_id);
            world.commands().entity(entity).despawn();
        }
        for (entity_uuid, entity) in uuid_map.drain() {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_common", "Despawning Entity {} associated with EntityUuid {:?}!", entity, entity_uuid);
            world.commands().entity(entity).despawn();
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub(super) trait InstanceData: Component + TypePath {
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

impl InstanceData for EntityId {
    type Relationship = EntityOfInstance;

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
    type Relationship = EntityOfInstance;

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
