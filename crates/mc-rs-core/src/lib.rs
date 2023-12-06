#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bevy::prelude::*;

// Re-export mc-rs-protocol
pub use mc_rs_protocol::{types::*, versions};

pub mod components;
pub mod events;
pub mod resources;
pub mod schedule;
pub mod sounds;

/// A plugin that adds core systems and resources.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        components::setup(app);
        events::setup(app);
        resources::setup(app);
        schedule::setup(app);
        sounds::setup(app);
    }
}
