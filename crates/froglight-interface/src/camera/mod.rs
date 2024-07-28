//! Cameras and camera-related systems.

use bevy::{
    app::{App, Plugin},
    prelude::{Component, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};

mod model;
pub use model::{ModelCamera, ModelCameraBundle, ModelCameraLayer};

mod overlay;
pub use overlay::{OverlayCamera, OverlayCameraBundle, OverlayCameraLayer};

mod ui;
pub use ui::{UiCamera, UiCameraBundle};

mod world;
pub use world::{WorldCamera, WorldCameraBundle};

/// A marker [`Component`] used to identify cameras.
///
/// Cameras are rendered in the following order:
/// 1. [`WorldCamera`]
///   - Order: `4`
///   - Layer: `0`
/// 2. [`ModelCamera`]
///   - Order: `8`
///   - Layer: `2`
/// 3. [`UiCamera`]
///   - Order: `12`
///   - Layer: `0`
/// 4. [`OverlayCamera`]
///   - Order: `16`
///   - Layer: `2`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Default, Component)]
pub struct FroglightCamera;

/// A [`Plugin`] that creates and sets up cameras.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        model::build(app);
        overlay::build(app);
        ui::build(app);
        world::build(app);
    }
}
