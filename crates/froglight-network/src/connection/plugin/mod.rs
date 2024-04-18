use bevy_app::{App, Plugin};

pub mod events;
mod systems;
mod traits;

/// The `Connection` Froglight plugin.
///
/// Adds networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        // Add events
        events::build(app);

        // Add systems
        systems::build(app);
    }
}
