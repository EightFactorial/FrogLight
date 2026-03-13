use alloc::borrow::Cow;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::prelude::*;

/// Adds methods for building [`GameCommands`] to an [`App`].
pub trait AddGameCommand {
    /// Add a [`GameCommand`] with a system to execute.
    fn add_game_command<S, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        system: S,
    ) -> &mut Self
    where
        S: IntoSystem<In<GameCommand>, (), Marker> + Send + Sync + 'static;

    /// Add a [`GameCommand`] to the [`CommandGraph`].
    fn add_game_command_node<In: SystemInput>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
    ) -> &mut Self;
}

impl AddGameCommand for App {
    #[allow(unused, reason = "Only used if tracing is enabled.")]
    fn add_game_command<S, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        system: S,
    ) -> &mut Self
    where
        S: IntoSystem<In<GameCommand>, (), Marker> + Send + Sync + 'static,
    {
        let command = command.into();
        self.add_game_command_node::<<S::System as System>::In>(command.clone());

        let system_id = self.world_mut().register_system(system);
        self.add_observer(move |mut event: On<GameCommand>, world: &mut World| {
            if event.command() == command.as_ref() {
                if event.arguments().is_none() {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(target: "froglight_brigadier", "Command \"{command}\" has no arguments, did another command take them?");
                    return;
                }

                let cmd = GameCommand::new_with(event.entity(), command.clone(), event.take_arguments());
                if let Err(err) = world.run_system_with(system_id, cmd) {
                    #[cfg(feature = "tracing")]
                    tracing::error!(target: "froglight_brigadier", "Error executing command \"{command}\": {err}");
                }
            }
        })
    }

    fn add_game_command_node<In: SystemInput>(
        &mut self,
        _command: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------
