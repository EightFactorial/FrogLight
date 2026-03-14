//! TODO

use alloc::borrow::Cow;

use bevy_app::prelude::*;

use crate::{builder::GameCommandBuilder, prelude::CommandGraph};

/// Adds methods for building [`GameCommands`] to an [`App`].
pub trait AddGameCommand {
    /// Build and add a [`GameCommand`] to the [`App`].
    ///
    /// # Panics
    ///
    /// Panics if a duplicate command is added.
    fn add_game_command(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        builder: impl FnOnce(GameCommandBuilder<'_, ()>),
    ) -> &mut Self;
}

impl AddGameCommand for App {
    fn add_game_command(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        builder: impl FnOnce(GameCommandBuilder<'_, ()>),
    ) -> &mut Self {
        // Register an empty function for the command
        let command = command.into();
        let mut graph = self.world_mut().get_resource_or_init::<CommandGraph>();
        let entrypoint = graph.register_function(command.clone(), None).expect("TODO");

        // Build the command function
        let mut function = None;
        (builder)(GameCommandBuilder::new(&mut graph, &mut function, entrypoint));

        // Update the command graph with the new function
        let mut graph = self.world_mut().get_resource_or_init::<CommandGraph>();
        let (_, f) = CommandGraph::as_commands_mut(&mut graph).get_mut(&command).unwrap();
        *f = function;

        self
    }
}
