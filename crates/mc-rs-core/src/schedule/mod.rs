use bevy::prelude::*;

pub mod set;
pub mod state;

pub(super) fn configure(app: &mut App) {
    state::configure(app);
    set::configure(app);
}
