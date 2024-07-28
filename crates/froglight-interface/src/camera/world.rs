use bevy::{
    app::App,
    core::Name,
    prelude::{Bundle, Camera, Camera3dBundle, Component, ReflectComponent, ReflectDefault},
    reflect::Reflect,
    render::{
        camera::CameraOutputMode,
        view::{Layer, RenderLayers},
    },
};

use super::FroglightCamera;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<WorldCamera>();
    app.world_mut().spawn(WorldCameraBundle::default());
}

/// A marker [`Component`] used to identify the world camera.
///
/// This camera displays the anything that exists in the world.
///
/// This includes other entities, the terrain, and anything else that
/// has a physical presence in the world not attached to the player.
///
/// For the player's arms and held items, see
/// [`ModelCamera`](super::ModelCamera).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Default, Component)]
pub struct WorldCamera;

impl WorldCamera {
    /// The order in which the [`WorldCamera`] is rendered.
    pub const CAMERA_ORDER: isize = 4;

    /// The name of the [`WorldCamera`].
    pub const CAMERA_NAME: &'static str = "World Camera";

    /// The render layer for the [`WorldCamera`].
    pub const RENDER_LAYER: Layer = 0;

    /// The [`RenderLayers`] for the [`WorldCamera`].
    ///
    /// By default, all entities are rendered on layer `0`.
    pub const RENDER_LAYERS: RenderLayers = RenderLayers::layer(Self::RENDER_LAYER);
}

/// A bundle that creates a [`WorldCamera`] and sets it up.
#[derive(Bundle)]
pub struct WorldCameraBundle {
    /// Mark the camera as a [`FroglightCamera`].
    pub froglight_camera: FroglightCamera,
    /// Mark the camera as a [`WorldCamera`].
    pub world_camera: WorldCamera,
    /// Add the [`Camera3dBundle`] components.
    pub camera_bundle: Camera3dBundle,
    /// Set the render layers for the camera.
    pub render_layers: RenderLayers,
    /// The name of the camera.
    pub name: Name,
}

impl Default for WorldCameraBundle {
    fn default() -> Self {
        Self {
            froglight_camera: FroglightCamera,
            world_camera: WorldCamera,
            camera_bundle: Camera3dBundle {
                camera: Camera {
                    order: WorldCamera::CAMERA_ORDER,
                    output_mode: CameraOutputMode::Skip,
                    ..Default::default()
                },
                ..Default::default()
            },
            render_layers: WorldCamera::RENDER_LAYERS,
            name: Name::new(WorldCamera::CAMERA_NAME),
        }
    }
}
