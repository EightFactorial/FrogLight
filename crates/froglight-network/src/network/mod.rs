//! Systems for networking and handling network packets.

use bevy_app::{App, Plugin};
use froglight_protocol::common::{ChunkPosition, Difficulty, GameMode};

mod channel;
pub use channel::{ConnectionChannel, PacketChannel};

mod event;
pub use event::{NetworkErrorEvent, ServerStatusResponse};

mod networking;
pub use networking::ConnectionTrait;

mod systemset;
pub use systemset::{NetworkPostUpdateSet, NetworkPreUpdateSet};

mod task;
pub use task::{ConnectionTask, ConnectionTaskResult, PolledTask, StatusTask, StatusTaskResult};

/// The `Network` Froglight plugin.
///
/// Adds networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        systemset::build(app);

        event::build(app);
        task::build(app);

        app.init_resource::<Difficulty>().register_type::<Difficulty>();
        app.register_type::<ChunkPosition>().register_type::<GameMode>();
    }
}
