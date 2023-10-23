use bevy::prelude::*;

pub use mc_rs_protocol::{types::*, versions};

pub mod blocks;
pub mod components;
pub mod resources;
pub mod schedule;
pub mod world;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        schedule::configure(app);

        blocks::add_systems(app);
        world::add_systems(app);
    }
}
