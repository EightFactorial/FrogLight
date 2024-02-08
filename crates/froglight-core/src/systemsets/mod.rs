//! [`SystemSets`](bevy::prelude::SystemSet) used by all `FrogLight` crates.

use bevy::prelude::*;

mod loading;
pub use loading::LoadingScreenUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { loading::build(app); }
