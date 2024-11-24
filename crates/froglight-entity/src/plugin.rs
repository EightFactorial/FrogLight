use bevy_app::{App, Plugin};

/// The `Entity` plugin for Froglight.
///
/// Adds entities and components for querying entities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, _app: &mut App) {}
}
