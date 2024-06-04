//! Systems for networking and handling network packets.

use bevy_app::{App, Plugin};
use froglight_protocol::common::{ChunkPosition, Difficulty, GameMode};

mod systemsets;
pub use systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet};

mod tasks;
pub use tasks::{ConnectionTask, ConnectionTaskResult, StatusTask, StatusTaskResult};

/// The `Network` Froglight plugin.
///
/// Adds networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        systemsets::build(app);

        tasks::build(app);

        app.init_resource::<Difficulty>().register_type::<Difficulty>();
        app.register_type::<ChunkPosition>().register_type::<GameMode>();
    }
}
