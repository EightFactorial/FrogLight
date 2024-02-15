//! Settings loading and saving.
use bevy::prelude::*;

mod resourcepack;
pub use resourcepack::ResourcePackSettings;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { resourcepack::build(app); }
