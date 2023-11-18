use bevy::prelude::*;

mod scale;
pub use scale::*;

pub(super) fn setup(app: &mut App) { scale::setup(app); }
