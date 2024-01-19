//! [`SystemSets`](bevy::prelude::SystemSet) used by all `FrogLight` crates.

use bevy::prelude::*;

pub mod loading;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) { loading::setup(app); }
