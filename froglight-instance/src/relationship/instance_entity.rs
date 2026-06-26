use alloc::vec::Vec;
use core::ops::Deref;

use bevy_ecs::{
    component::{Component, ComponentInfo},
    entity::Entity,
    lifecycle::HookContext,
    reflect::{AppTypeRegistry, ReflectComponent},
    world::DeferredWorld,
};
use bevy_reflect::Reflect;

use crate::instance::reflect::ReflectSession;

/// An [`Entity`] that is part of a
/// [`SessionInstance`](crate::prelude::SessionInstance).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Reflect)]
#[reflect(Debug, Clone, PartialEq, PartialOrd, Hash, Component)]
#[component(on_insert = Self::on_insert, on_discard = Self::on_discard)]
pub struct PartOfInstance(Entity);

impl PartOfInstance {
    /// Create a new [`PartOfInstance`] [`Component`] using a
    /// [`SessionInstance`](crate::prelude::SessionInstance) [`Entity`].
    #[inline]
    #[must_use]
    pub const fn new(instance: Entity) -> Self { Self(instance) }

    /// Get the [`Entity`] of the
    /// [`SessionInstance`](crate::prelude::SessionInstance) this entity is part
    /// of.
    #[inline]
    #[must_use]
    pub const fn instance(self) -> Entity { self.0 }

    fn on_insert(mut world: DeferredWorld, ctx: HookContext) {
        let Ok(entity) = world.get_entity(ctx.entity) else { return };

        // Get all of the components on the current entity.
        let iter = entity.archetype().components().iter();
        let iter = iter.filter_map(|id| world.components().get_info(*id));
        let components: Vec<_> = iter.filter_map(ComponentInfo::type_id).collect();

        let registry = world.resource::<AppTypeRegistry>().clone();
        let registry = registry.read();

        // Run the `on_insert` hook for `ReflectSession` components.
        for reflect in
            components.into_iter().filter_map(|id| registry.get_type_data::<ReflectSession>(id))
        {
            reflect.on_insert(ctx.entity, world.reborrow());
        }
    }

    fn on_discard(mut world: DeferredWorld, ctx: HookContext) {
        let Ok(entity) = world.get_entity(ctx.entity) else { return };

        // Get all of the components on the current entity.
        let iter = entity.archetype().components().iter();
        let iter = iter.filter_map(|id| world.components().get_info(*id));
        let components: Vec<_> = iter.filter_map(ComponentInfo::type_id).collect();

        let registry = world.resource::<AppTypeRegistry>().clone();
        let registry = registry.read();

        // Run the `on_discard` hook for `ReflectSession` components.
        for reflect in
            components.into_iter().filter_map(|id| registry.get_type_data::<ReflectSession>(id))
        {
            reflect.on_discard(ctx.entity, world.reborrow());
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Deref for PartOfInstance {
    type Target = Entity;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<Entity> for PartOfInstance {
    #[inline]
    fn from(value: Entity) -> Self { Self::new(value) }
}
impl From<PartOfInstance> for Entity {
    #[inline]
    fn from(value: PartOfInstance) -> Self { value.instance() }
}
