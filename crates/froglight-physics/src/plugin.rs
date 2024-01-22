use bevy::prelude::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins as XPBDPhysicsPlugins;

/// The [`Plugin`] for the [`froglight-physics`](crate) crate.
///
/// Adds entity physics and collision.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Add the `bevy_xpbd_3d` plugin.
        app.add_plugins(XPBDPhysicsPlugins::default());
    }
}
