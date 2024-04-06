use bevy::app::{App, Plugin};

/// The `Connection` Froglight plugin.
///
/// Adds networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, _app: &mut App) {}
}
