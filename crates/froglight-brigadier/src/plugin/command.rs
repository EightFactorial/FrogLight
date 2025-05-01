#[cfg(not(feature = "std"))]
use alloc::string::String;

use bevy_ecs::{
    system::{EntityCommand, EntityCommands},
    world::EntityWorldMut,
};
use derive_more::{Deref, From};

use super::BrigadierEvent;

/// An Brigadier command executed by an [`Entity`].
///
/// Internally sends a [`BrigadierEvent`] to be handled later.
#[derive(Debug, Clone, PartialEq, Eq, Hash, From, Deref)]
pub struct BrigadierCommand(String);

impl BrigadierCommand {
    /// Create a new [`BrigadierCommand`] with the given command.
    #[inline]
    #[must_use]
    pub const fn new(command: String) -> Self { Self(command) }

    /// Create a new [`BrigadierCommand`] with the given command.
    #[inline]
    #[must_use]
    pub fn new_from(command: impl Into<String>) -> Self { Self::new(command.into()) }
}

impl EntityCommand for BrigadierCommand {
    fn apply(self, mut entity: EntityWorldMut) {
        let event = BrigadierEvent::new(entity.id(), self.0);
        entity.world_scope(|world| world.send_event(event));
    }
}

/// A trait for running Brigadier commands.
pub trait BrigadierCommands {
    /// Run a [`BrigadierCommand`] as an [`Entity`].
    fn run_command(&mut self, command: impl Into<String>) -> &mut Self;
}

impl BrigadierCommands for EntityCommands<'_> {
    fn run_command(&mut self, command: impl Into<String>) -> &mut Self {
        self.queue(BrigadierCommand::new_from(command));
        self
    }
}
impl BrigadierCommands for EntityWorldMut<'_> {
    fn run_command(&mut self, command: impl Into<String>) -> &mut Self {
        let entity = self.id();
        self.world_scope(|world| world.send_event(BrigadierEvent::new(entity, command.into())));
        self
    }
}
