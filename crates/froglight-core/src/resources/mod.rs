//! [`Resources`](bevy::prelude::Resource) used by all `FrogLight` crates.

use bevy_app::App;

mod interface;
pub use interface::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { interface::setup(app); }
