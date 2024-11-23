//! Systems for networking and handling network packets.

use bevy_app::{App, Plugin};
use froglight_protocol::common::{ChunkPosition, Difficulty, GameMode};

pub mod channel;
pub use channel::{BevyConnectionChannel, ChannelRecvPacket, ChannelSendPacket};

mod connect;
pub use connect::ConnectTrait;

mod systemset;
pub use systemset::NetworkPostUpdateSet;

mod task;
pub use task::{
    ConnectionClosedEvent, ConnectionErrorEvent, ConnectionTask, PolledTask, StatusResponseEvent,
    StatusTask,
};

/// The `Network` Froglight plugin.
///
/// Adds networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Difficulty>()
            .register_type::<GameMode>()
            .register_type::<ChunkPosition>();

        systemset::build(app);
        task::build(app);
    }
}
