//! [`Events`](bevy_ecs::event::Event) for Froglight.

use bevy_app::App;

mod assets;
pub use assets::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { assets::build(app); }
