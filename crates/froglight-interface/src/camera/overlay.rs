use bevy::{
    app::App,
    core::Name,
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::DeferredWorld,
    },
    prelude::{
        Bundle, Camera, Camera2dBundle, ClearColorConfig, Component, Entity, ReflectComponent,
        ReflectDefault,
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
    app.register_type::<OverlayCamera>().register_type::<OverlayCameraLayer>();
    app.world_mut().spawn(OverlayCameraBundle::default());
}

/// A marker [`Component`] used to identify the overlay camera.
///
/// This camera displays anything that should be rendered on top of everything
/// else.
///
/// This includes the inventory and crafting menus,
/// the pause menu, and the death screen.
///
/// For UI elements, see [`UiCamera`](super::UiCamera).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Default, Component)]
pub struct OverlayCamera;

impl OverlayCamera {
    /// The order in which the [`OverlayCamera`] is rendered.
    pub const CAMERA_ORDER: isize = 16;

    /// The name of the [`OverlayCamera`].
    pub const CAMERA_NAME: &'static str = "Overlay Camera";

    /// The render layer for the [`OverlayCamera`].
    pub const RENDER_LAYER: Layer = 2;

    /// The [`RenderLayers`] for the [`OverlayCamera`].
    ///
    /// Any entities that should be rendered by this camera must be
    /// on this layer.
    pub const RENDER_LAYERS: RenderLayers = RenderLayers::layer(Self::RENDER_LAYER);
}

/// A bundle that creates an [`OverlayCamera`] and sets it up.
#[derive(Bundle)]
pub struct OverlayCameraBundle {
    /// Mark the camera as a [`FroglightCamera`].
    pub froglight_camera: FroglightCamera,
    /// Mark the camera as an [`OverlayCamera`].
    pub overlay_camera: OverlayCamera,
    /// Add the [`Camera2dBundle`] components.
    pub camera_bundle: Camera2dBundle,
    /// Set the render layers for the camera.
    pub render_layers: RenderLayers,
    /// The name of the camera.
    pub name: Name,
}

impl Default for OverlayCameraBundle {
    fn default() -> Self {
        Self {
            froglight_camera: FroglightCamera,
            overlay_camera: OverlayCamera,
            camera_bundle: Camera2dBundle {
                camera: Camera {
                    order: OverlayCamera::CAMERA_ORDER,
                    output_mode: CameraOutputMode::Write {
                        blend_state: None,
                        clear_color: ClearColorConfig::Default,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            render_layers: OverlayCamera::RENDER_LAYERS,
            name: Name::new(OverlayCamera::CAMERA_NAME),
        }
    }
}

/// A [`Component`] that adds the [`OverlayCamera`] render layer to an entity.
///
/// This component is not required, it's just for convenience.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect)]
#[reflect(Default, Component)]
pub struct OverlayCameraLayer;

impl Component for OverlayCameraLayer {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(OverlayCameraLayer::on_insert).on_remove(OverlayCameraLayer::on_remove);
    }
}

// TODO: Remove `RenderLayers::clone`?
impl OverlayCameraLayer {
    /// Adds the [`RenderLayers`] required by the [`OverlayCamera`].
    fn on_insert(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(mut layers) = world.get_mut::<RenderLayers>(entity) {
            // If `RenderLayers` does not contain `OverlayCamera::RENDER_LAYER`, add it.
            if !layers.intersects(&OverlayCamera::RENDER_LAYERS) {
                *layers = layers.clone().with(OverlayCamera::RENDER_LAYER);
            }
        } else {
            // Add `OverlayCamera::RENDER_LAYERS` if `RenderLayers` does not exist.
            world.commands().entity(entity).insert(OverlayCamera::RENDER_LAYERS);
        }
    }

    /// Removes the [`RenderLayers`] required by the [`OverlayCamera`].
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(mut layers) = world.get_mut::<RenderLayers>(entity) {
            // If `RenderLayers` contains `OverlayCamera::RENDER_LAYER`, remove the layer.
            if layers.intersects(&OverlayCamera::RENDER_LAYERS) {
                let without = layers.clone().without(OverlayCamera::RENDER_LAYER);

                if without.iter().count() == 0 {
                    // If `RenderLayers` is empty, remove it.
                    world.commands().entity(entity).remove::<RenderLayers>();
                } else {
                    *layers = without;
                }
            }
        }
    }
}
