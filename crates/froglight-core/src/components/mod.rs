//! [`Components`](bevy::prelude::Component) used by all `FrogLight` crates.

use bevy_app::App;

mod camera;
pub use camera::PlayerCamera;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { camera::setup(app); }
