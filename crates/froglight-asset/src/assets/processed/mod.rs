//! Processed assets that can be used directly in the game.

use bevy_app::App;

pub(crate) mod sound_event;
pub use sound_event::SoundEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { sound_event::build(app); }
