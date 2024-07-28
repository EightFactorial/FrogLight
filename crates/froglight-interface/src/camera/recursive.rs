use bevy::{
    app::App,
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::DeferredWorld,
    },
    prelude::{Children, Component, Entity, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};

use super::{ModelCameraLayer, OverlayCameraLayer};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<RecursiveCameraLayer<ModelCameraLayer>>()
        .register_type::<RecursiveCameraLayer<OverlayCameraLayer>>();
}

/// A [`Component`] that recursively adds
/// [`RenderLayers`](bevy::render::view::RenderLayers) to entities.
///
/// # Note
/// This ***does not check*** for new children after the initial insert!
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect)]
#[reflect(Default, Component)]
pub struct RecursiveCameraLayer<C: RecursiveLayer>(#[reflect(ignore)] std::marker::PhantomData<C>);

impl<C: RecursiveLayer> Component for RecursiveCameraLayer<C> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks
            .on_insert(RecursiveCameraLayer::<C>::on_insert)
            .on_remove(RecursiveCameraLayer::<C>::on_remove);
    }
}

impl<C: RecursiveLayer> RecursiveCameraLayer<C> {
    /// Recursively adds `C`'s
    /// [`RenderLayers`](bevy::render::view::RenderLayers) to the entity and its
    /// children.
    fn on_insert(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        Self::recursively_insert(&mut world, entity);
    }

    /// Recursively inserts `C` into the entity and its children.
    fn recursively_insert(world: &mut DeferredWorld, entity: Entity) {
        // Add `C` if it does not exist.
        world.commands().entity(entity).insert(C::default());

        // Recursively add `C` to children.
        if let Some(children) =
            world.get::<Children>(entity).map(|c| c.iter().copied().collect::<Vec<_>>())
        {
            for child in children {
                Self::recursively_insert(world, child);
            }
        }
    }

    /// Recursively removes `C`'s
    /// [`RenderLayers`](bevy::render::view::RenderLayers) from the entity and
    /// its children.
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        Self::recursively_remove(&mut world, entity);
    }

    /// Recursively removes `C` from the entity and its children.
    fn recursively_remove(world: &mut DeferredWorld, entity: Entity) {
        // Remove `C` if it exists.
        world.commands().entity(entity).remove::<C>();

        // Recursively remove `C` from children.
        if let Some(children) =
            world.get::<Children>(entity).map(|c| c.iter().copied().collect::<Vec<_>>())
        {
            for child in children {
                Self::recursively_remove(world, child);
            }
        }
    }
}

use sealed::RecursiveLayer;
mod sealed {
    use bevy::{prelude::Component, reflect::Reflect};

    use crate::camera::{ModelCameraLayer, OverlayCameraLayer};

    #[allow(unreachable_pub)]
    pub trait RecursiveLayer: 'static + Send + Sync + Default + Reflect + Component {}
    impl RecursiveLayer for ModelCameraLayer {}
    impl RecursiveLayer for OverlayCameraLayer {}
}
