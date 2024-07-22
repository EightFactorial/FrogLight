use bevy::{
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::DeferredWorld,
    },
    prelude::*,
    reflect::Reflect,
};

/// The child [`Entity`] that [holds the loading screen](LoadingScreenHolder).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Component)]
#[reflect(Component)]
pub struct LoadingScreenChild(pub Entity);

/// The root [`Entity`] that holds the loading screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct LoadingScreenHolder;

impl Component for LoadingScreenHolder {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(Self::on_insert).on_remove(Self::on_remove);
    }
}

impl LoadingScreenHolder {
    /// The `on_insert` [`ComponentHook`](ComponentHooks) for the
    /// [`LoadingScreenHolder`].
    ///
    /// Spawns a loading screen when [`LoadingScreenHolder`] is inserted.
    fn on_insert(_world: DeferredWorld, _entity: Entity, _: ComponentId) {}

    /// The `on_remove` [`ComponentHook`](ComponentHooks) for the
    /// [`LoadingScreenHolder`].
    ///
    /// Despawns the entity and all its descendants when [`LoadingScreenHolder`]
    /// is removed.
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        world.commands().entity(entity).despawn_recursive();
    }
}
