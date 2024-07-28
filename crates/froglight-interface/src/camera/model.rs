use bevy::{
    app::App,
    core::Name,
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::DeferredWorld,
    },
    prelude::{
        Bundle, Camera, Camera3dBundle, Component, Entity, ReflectComponent, ReflectDefault,
    },
    reflect::Reflect,
    render::{
        camera::CameraOutputMode,
        view::{Layer, RenderLayers},
    },
};

use super::FroglightCamera;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ModelCamera>().register_type::<ModelCameraLayer>();
    app.world_mut().spawn(ModelCameraBundle::default());
}

/// A marker [`Component`] used to identify the model camera.
///
/// This camera displays the first-person view of the player.
///
/// This includes the player's arms, any held items,
/// and anything else attached to the player that should be visible in
/// first-person.
///
/// For the world, see [`WorldCamera`](super::WorldCamera).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Default, Component)]
pub struct ModelCamera;

impl ModelCamera {
    /// The order in which the [`ModelCamera`] is rendered.
    pub const CAMERA_ORDER: isize = 8;

    /// The name of the [`ModelCamera`].
    pub const CAMERA_NAME: &'static str = "Model Camera";

    /// The render layer for the [`ModelCamera`].
    pub const RENDER_LAYER: Layer = 2;

    /// The [`RenderLayers`] for the [`ModelCamera`].
    ///
    /// Any entities that should be rendered by this camera must be
    /// on this layer.
    pub const RENDER_LAYERS: RenderLayers = RenderLayers::layer(Self::RENDER_LAYER);
}

/// A bundle that creates a [`ModelCamera`] and sets it up.
#[derive(Bundle)]
pub struct ModelCameraBundle {
    /// Mark the camera as a [`FroglightCamera`].
    pub froglight_camera: FroglightCamera,
    /// Mark the camera as a [`ModelCamera`].
    pub model_camera: ModelCamera,
    /// Add the [`Camera3dBundle`] components.
    pub camera_bundle: Camera3dBundle,
    /// Set the render layers for the camera.
    pub render_layers: RenderLayers,
    /// The name of the camera.
    pub name: Name,
}

impl Default for ModelCameraBundle {
    fn default() -> Self {
        Self {
            froglight_camera: FroglightCamera,
            model_camera: ModelCamera,
            camera_bundle: Camera3dBundle {
                camera: Camera {
                    order: ModelCamera::CAMERA_ORDER,
                    output_mode: CameraOutputMode::Skip,
                    ..Default::default()
                },
                ..Default::default()
            },
            render_layers: ModelCamera::RENDER_LAYERS,
            name: Name::new(ModelCamera::CAMERA_NAME),
        }
    }
}

/// A [`Component`] that adds the [`ModelCamera::RENDER_LAYER`] to an entity.
///
/// If the entity has no existing [`RenderLayers`],
/// it will only be rendered by the [`ModelCamera`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect)]
#[reflect(Default, Component)]
pub struct ModelCameraLayer;

impl Component for ModelCameraLayer {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(ModelCameraLayer::on_insert).on_remove(ModelCameraLayer::on_remove);
    }
}

// TODO: Remove `RenderLayers::clone`?
impl ModelCameraLayer {
    /// Adds the [`RenderLayers`] required by the [`ModelCamera`].
    fn on_insert(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(mut layers) = world.get_mut::<RenderLayers>(entity) {
            // If `RenderLayers` does not contain `ModelCamera::RENDER_LAYER`, add it.
            if !layers.intersects(&ModelCamera::RENDER_LAYERS) {
                *layers = layers.clone().with(ModelCamera::RENDER_LAYER);
            }
        } else {
            // Add `ModelCamera::RENDER_LAYERS` if `RenderLayers` does not exist.
            world.commands().entity(entity).insert(ModelCamera::RENDER_LAYERS);
        }
    }

    /// Removes the [`RenderLayers`] required by the [`ModelCamera`].
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(mut layers) = world.get_mut::<RenderLayers>(entity) {
            // If `RenderLayers` contains `ModelCamera::RENDER_LAYER`, remove the layer.
            if layers.intersects(&ModelCamera::RENDER_LAYERS) {
                let without = layers.clone().without(ModelCamera::RENDER_LAYER);
                if without.iter().next().is_none() {
                    // If `RenderLayers` is empty, remove it.
                    world.commands().entity(entity).remove::<RenderLayers>();
                } else {
                    *layers = without;
                }
            }
        }
    }
}
