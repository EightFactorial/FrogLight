//! [`Events`](bevy_ecs::event::Event) used by all `FrogLight` crates.

use bevy_app::App;

mod assets;
pub use assets::{ResourcePackFinishedLoadingEvent, ResourcePackStartLoadingEvent};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { assets::build(app); }
