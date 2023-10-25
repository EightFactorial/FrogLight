use bevy::prelude::*;

use bevy_rapier3d::prelude::RapierPhysicsPlugin;
pub use mc_rs_protocol::{types::*, versions};

pub mod blocks;
pub mod components;
pub mod resources;
pub mod schedule;
pub mod world;

mod net_event;
pub use net_event::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Add Rapier physics plugins
        app.add_plugins(RapierPhysicsPlugin::<()>::default());

        // use bevy_rapier3d::render::RapierDebugRenderPlugin;
        // app.add_plugins(RapierDebugRenderPlugin::default());

        schedule::configure(app);
        net_event::configure(app);

        blocks::add_systems(app);
        world::add_systems(app);
    }
}
