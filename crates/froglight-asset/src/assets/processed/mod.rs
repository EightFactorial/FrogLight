//! Assets processed into usable forms.

use bevy_app::App;

pub mod sound;
pub use sound::{SoundMap, SoundSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { sound::build(app); }
