use bevy_app::{App, Plugin};

mod channel;
pub use channel::{current::ConnectionChannel, legacy::LegacyChannel};

mod events;
pub use events::*;

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
