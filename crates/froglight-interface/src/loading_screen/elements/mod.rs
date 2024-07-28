//! UI elements that make up the loading screen.

use bevy::app::App;

mod background;
pub use background::LoadingScreenBackground;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { background::build(app); }
