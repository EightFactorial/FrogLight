//! [`Resources`](bevy_ecs::system::Resource) for Froglight.

use bevy_app::App;

mod interface;
pub use interface::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { interface::build(app); }
