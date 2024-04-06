//! [`Components`](bevy_ecs::component::Component) for Froglight.

use bevy_app::App;

mod client;
pub use client::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { client::build(app); }
