use bevy_app::{App, Plugin};
use froglight_protocol::versions::{v1_20_0::V1_20_0, v1_20_2::V1_20_2, v1_20_3::V1_20_3};

pub mod channels;
pub use channels::{current::PacketChannel, legacy::LegacyPacketChannel};

pub mod events;

pub mod handler;
pub use handler::ConnectionHandler;

mod misc;
pub use misc::{ConnectionMarker, ConnectionTask, StatusTask};

mod resources;
pub use resources::{ConfigPlugins, LoginPlugins, PlayPlugins};

pub mod systemsets;

/// The `Connection` Froglight plugin.
///
/// Adds networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        systemsets::build(app);
        events::build(app);

        // Insert resources
        resources::build(app);

        // Build task systems
        ConnectionTask::build(app);
        StatusTask::build(app);

        // Build ConnectionHandler systems
        <V1_20_0 as ConnectionHandler>::build(app);
        <V1_20_2 as ConnectionHandler>::build(app);
        <V1_20_3 as ConnectionHandler>::build(app);
    }
}
