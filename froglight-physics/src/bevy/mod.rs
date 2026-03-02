//! TODO

use bevy_app::{App, Plugin};

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, _app: &mut App) {}
}
