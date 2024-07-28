use bevy::{
    app::App,
    core::Name,
    prelude::{Bundle, Camera, Camera2dBundle, Component, ReflectComponent, ReflectDefault},
    reflect::Reflect,
    render::{
        camera::CameraOutputMode,
        view::{Layer, RenderLayers},
    },
};

use super::FroglightCamera;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<UiCamera>();
    app.world_mut().spawn(UiCameraBundle::default());
}

/// A marker [`Component`] used to identify the UI camera.
///
/// This camera displays the user interface.
///
/// This includes the player's health bar, the hotbar, and any other
/// UI elements that should be visible on the screen that do not
/// fill the entire screen.
///
/// For inventories and menus, see [`OverlayCamera`](super::OverlayCamera).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Default, Component)]
pub struct UiCamera;

impl UiCamera {
    /// The order in which the [`UiCamera`] is rendered.
    pub const CAMERA_ORDER: isize = 12;

    /// The name of the [`UiCamera`].
    pub const CAMERA_NAME: &'static str = "UI Camera";

    /// The render layer for the [`UiCamera`].
    pub const RENDER_LAYER: Layer = 0;

    /// The [`RenderLayers`] for the [`UiCamera`].
    ///
    /// By default, all entities are rendered on layer `0`.
    pub const RENDER_LAYERS: RenderLayers = RenderLayers::layer(Self::RENDER_LAYER);
}

/// A bundle that creates an [`UiCamera`] and sets it up.
#[derive(Bundle)]
pub struct UiCameraBundle {
    /// Mark the camera as a [`FroglightCamera`].
    pub froglight_camera: FroglightCamera,
    /// Mark the camera as a [`UiCamera`].
    pub ui_camera: UiCamera,
    /// Add the [`Camera2dBundle`] components.
    pub camera_bundle: Camera2dBundle,
    /// Set the render layers for the camera.
    pub render_layers: RenderLayers,
    /// The name of the camera.
    pub name: Name,
}

impl Default for UiCameraBundle {
    fn default() -> Self {
        Self {
            froglight_camera: FroglightCamera,
            ui_camera: UiCamera,
            camera_bundle: Camera2dBundle {
                camera: Camera {
                    order: UiCamera::CAMERA_ORDER,
                    output_mode: CameraOutputMode::Skip,
                    ..Default::default()
                },
                ..Default::default()
            },
            render_layers: UiCamera::RENDER_LAYERS,
            name: Name::new(UiCamera::CAMERA_NAME),
        }
    }
}
