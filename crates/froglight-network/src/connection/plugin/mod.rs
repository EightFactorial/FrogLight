#![allow(dead_code)]
use bevy_app::{App, Plugin};

pub mod channels;
pub use channels::{current::PacketChannel, legacy::LegacyPacketChannel};

pub mod handler;
pub use handler::ConnectionHandler;

mod misc;
pub use misc::{ConnectionMarker, ConnectionTask, StatusTask};

mod resources;
pub use resources::{ConfigPlugins, LoginPlugins, PlayPlugins};

mod systemsets;

/// The `Connection` Froglight plugin.
///
/// Adds networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        resources::build(app);
        // TODO: events

        // Build ConnectionHandler systems
        // <V1_20_0 as ConnectionHandler>::build(app);
        // <V1_20_2 as ConnectionHandler>::build(app);
        // <V1_20_3 as ConnectionHandler>::build(app);
    }
}
