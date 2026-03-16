//! TODO

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{prelude::GameCommandEvent, set::CommandSet};

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrigadierPlugin;

impl Plugin for BrigadierPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCommandEvent>().register_type::<CommandSet>();
        app.init_resource::<CommandSet>().add_observer(Self::command_observer);
    }
}

impl BrigadierPlugin {
    /// An [`Observer`] that executes [`GameCommandEvent`]s.
    pub fn command_observer(event: On<GameCommandEvent>, world: &mut World) {
        world.resource_scope::<CommandSet, _>(|world, commands| {
            if let Err(err) = commands.execute(event.entity(), event.command(), world) {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to execute command \"{}\": {err:?}", event.command());
            }
        });
    }
}
