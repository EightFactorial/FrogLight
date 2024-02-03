//! [`Resources`](bevy::prelude::Resource) used by all `FrogLight` crates.
use bevy::prelude::*;

pub mod loading;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { loading::setup(app); }
