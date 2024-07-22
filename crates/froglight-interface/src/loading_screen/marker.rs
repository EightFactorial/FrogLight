use bevy::{
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::DeferredWorld,
    },
    prelude::*,
};

use super::{LoadingScreenChild, LoadingScreenHolder};

/// A loading screen displayed while the client is loading.
///
/// Spawns a child with a loading screen when inserted.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Default, Component)]
pub struct LoadingScreen;

impl Component for LoadingScreen {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(Self::on_insert).on_remove(Self::on_remove);
    }
}

impl LoadingScreen {
    /// The `on_insert` [`ComponentHook`](ComponentHooks) for the
    /// [`LoadingScreen`].
    ///
    /// If the entity has no [`LoadingScreenChild`] component, spawn
    /// a child with a [`LoadingScreenHolder`] component and use it in
    /// the [`LoadingScreenChild`] component.
    fn on_insert(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if world.get::<LoadingScreenChild>(entity).is_none() {
            // Spawn a child entity with a `LoadingScreenHolder` component.
            let child = world.commands().spawn(LoadingScreenHolder).id();
            // Attach the child entity to the parent entity.
            world.commands().entity(entity).insert(LoadingScreenChild(child)).add_child(child);
        } else {
            warn!("LoadingScreen \"{entity}\" already has a `LoadingScreenChild` component?");
        }
    }

    /// The `on_remove` [`ComponentHook`](ComponentHooks) for the
    /// [`LoadingScreen`].
    ///
    /// Despawns the entity in the [`LoadingScreenChild`] component.
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(child) = world.get::<LoadingScreenChild>(entity).copied() {
            // Despawn the child entity and all its descendants.
            world.commands().entity(*child).despawn_recursive();
        } else {
            warn!("LoadingScreen \"{entity}\" has no `LoadingScreenChild` component?");
        }
    }
}
