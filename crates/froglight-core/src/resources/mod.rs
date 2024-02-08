//! [`Resources`](bevy::prelude::Resource) used by all `FrogLight` crates.
use bevy::prelude::*;

mod loading;
pub use loading::LoadingScreenEnable;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { loading::setup(app); }
