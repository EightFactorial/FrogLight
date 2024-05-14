//! Networking systems and components.

use bevy::app::App;

mod systemsets;
pub use systemsets::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { systemsets::build(app); }
