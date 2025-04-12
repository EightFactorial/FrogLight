//! TODO

use bevy_ecs::{component::ComponentId, prelude::*, world::DeferredWorld};
use bevy_log::error;
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut};
use froglight_common::identifier::Identifier;
use hashbrown::{HashMap, hash_map::Entry};

mod reflect;
pub use reflect::{ReflectSubWorldSync, SubWorldSync};

/// A [`World`] that belongs to an [`Entity`].
#[derive(Debug, Deref, DerefMut, Component)]
#[component(
    on_insert = SubWorld::insert_hook,
    on_replace = SubWorld::remove_hook,
)]
pub struct SubWorld {
    identifier: Identifier,
    #[deref]
    #[deref_mut]
    world: World,
}

impl SubWorld {
    /// Get a reference to the [`SubWorld`]'s [`Identifier`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier { &self.identifier }

    /// Get a reference to the [`SubWorld`]'s [`World`].
    #[inline]
    #[must_use]
    pub const fn world(&self) -> &World { &self.world }

    /// Get a mutable reference to the [`SubWorld`]'s [`World`].
    #[inline]
    #[must_use]
    pub const fn world_mut(&mut self) -> &mut World { &mut self.world }
}

impl SubWorld {
    /// Create a new [`SubWorld`] from an [`Identifier`] and a [`World`].
    ///
    /// # Errors
    /// Returns an error if a [`SubWorld`] with the same [`Identifier`] exists.
    #[expect(clippy::result_unit_err)]
    pub fn from_world(identifier: Identifier, world: &mut World) -> Result<Self, ()> {
        // Return an error if the identifier is already in use.
        if world.get_resource_or_init::<SubWorlds>().contains_key(&identifier) {
            bevy_log::error!("A SubWorld with the identifier \"{identifier}\" already exists!");
            return Err(());
        }

        let mut sub_world = World::new();

        // Share the `AppTypeRegistry` with the `SubWorld`.
        let registry = world.resource::<AppTypeRegistry>();
        sub_world.insert_resource(registry.clone());

        // Use `SubWorldSync` to initialize the `SubWorld`.
        for (_, reflect) in registry.clone().read().iter_with_data::<ReflectSubWorldSync>() {
            reflect.init(&identifier, world, &mut sub_world);
        }

        Ok(Self { identifier, world: sub_world })
    }

    /// Run the registered `SubWorldSync::on_add` hooks.
    fn insert_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let registry = world.resource::<AppTypeRegistry>().clone();
        for (_, reflect) in registry.read().iter_with_data::<ReflectSubWorldSync>() {
            reflect.on_add(entity, &mut world);
        }
    }

    /// Run the registered `SubWorldSync::on_add` hooks.
    fn remove_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let registry = world.resource::<AppTypeRegistry>().clone();
        for (_, reflect) in registry.read().iter_with_data::<ReflectSubWorldSync>() {
            reflect.on_remove(entity, &mut world);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A map of [`Entity`]s to their respective [`SubWorld`]s
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource, Reflect)]
#[reflect(Debug, Default, PartialEq, Resource, SubWorldSync)]
pub struct SubWorlds(HashMap<Identifier, Entity>);

impl SubWorldSync for SubWorlds {
    /// Do nothing.
    fn initialize(_: &Identifier, _: &mut World, _: &mut World) {}

    /// Insert an [`Entity`]-[`Identifier`] relationship into the map.
    fn on_add(entity: Entity, world: &mut DeferredWorld) {
        if let Some(identifier) = world.get::<SubWorld>(entity).map(SubWorld::identifier).cloned() {
            if let Some(mut map) = world.get_resource_mut::<SubWorlds>() {
                match map.entry(identifier) {
                    Entry::Vacant(entry) => {
                        entry.insert(entity);
                    }
                    Entry::Occupied(entry) => {
                        error!("The SubWorld \"{}\" already exists!", entry.key());
                    }
                }
            }
        }
    }

    /// Remove an [`Entity`]-[`Identifier`] relationship from the map.
    fn on_remove(entity: Entity, world: &mut DeferredWorld) {
        if let Some(identifier) = world.get::<SubWorld>(entity).map(SubWorld::identifier).cloned() {
            world.get_resource_mut::<SubWorlds>().and_then(|mut map| map.remove(&identifier));
        }
    }
}
