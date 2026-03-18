//! TODO

use bevy_app::{App, Plugin};

use crate::prelude::*;

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform>().register_type::<PreviousTransform>();
        app.register_type::<Velocity>().register_type::<PreviousVelocity>();
        app.register_type::<Acceleration>().register_type::<PreviousAcceleration>();
        app.register_type::<OnGround>().register_type::<PreviousOnGround>();

        app.register_type::<PhysicsState>().register_type::<PhysicsController>();
    }
}
