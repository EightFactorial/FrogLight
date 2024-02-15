//! [`SystemSets`](bevy::prelude::SystemSet) used by all `FrogLight` crates.

use bevy::prelude::*;

mod loading;
pub use loading::LoadingScreenUpdateSet;

mod resourcepack;
pub use resourcepack::ResourcePackUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    loading::build(app);
    resourcepack::build(app);
}
