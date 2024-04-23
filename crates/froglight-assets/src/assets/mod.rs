//! Asset types and loaders.

use bevy_app::App;

pub mod resourcepack;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { resourcepack::build(app); }
