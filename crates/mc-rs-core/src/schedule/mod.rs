use bevy::prelude::*;

pub mod set;
pub mod state;

pub(super) fn setup(app: &mut App) {
    state::configure(app);
    set::configure(app);
}
