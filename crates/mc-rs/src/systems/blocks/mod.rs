use bevy::prelude::{App, Startup};

pub use self::{block::Blocks, state::BlockStates};

pub mod attributes;
pub mod block;
pub mod state;

/// Add the [Blocks] and [BlockStates] resources to the app.
pub(super) fn add_systems(app: &mut App) {
    app.add_systems(Startup, (Blocks::create, BlockStates::create));
}
