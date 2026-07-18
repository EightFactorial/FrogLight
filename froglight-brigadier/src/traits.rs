//! TODO

use alloc::{borrow::Cow, string::String};

use bevy_app::prelude::*;
use bevy_ecs::{error::ErrorContext, prelude::*};

use crate::{
    bundle::ArgumentBundle,
    prelude::{GameCommandCtx, GameCommandSet},
};

/// A extension trait adding [`App::add_game_command`] and
/// [`App::add_game_command_using`].
pub trait AppGameCommand {
    /// Add a game command the the [`App`].
    ///
    /// # Panics
    ///
    /// Panics if a command with the same name is already registered.
    #[inline]
    fn add_game_command<B: ArgumentBundle, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        system: impl IntoSystem<GameCommandCtx<B>, (), Marker> + 'static,
    ) where
        B::BundleData: Default,
    {
        self.add_game_command_using(command, B::BundleData::default(), system);
    }

    /// Add a game command the the [`App`].
    ///
    /// # Panics
    ///
    /// Panics if a command with the same name is already registered.
    fn add_game_command_using<B: ArgumentBundle, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        settings: B::BundleData,
        system: impl IntoSystem<GameCommandCtx<B>, (), Marker> + 'static,
    );
}

impl AppGameCommand for App {
    fn add_game_command_using<B: ArgumentBundle, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        settings: B::BundleData,
        system: impl IntoSystem<GameCommandCtx<B>, (), Marker> + 'static,
    ) {
        let system = self.world_mut().register_system_cached(system);
        if let Err(err) = self
            .world_mut()
            .get_resource_or_init::<GameCommandSet>()
            .register_command_using(command.into(), settings, system)
        {
            panic!("Failed to register command: {err:?}");
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An extension trait for [`Commands`] that adds [`Commands::game_command`].
pub trait CommandsGameCommand {
    /// Queue a game command to be executed by the given entity.
    #[inline]
    fn game_command(&mut self, entity: Entity, command: impl AsRef<str>) -> &mut Self {
        self.game_command_with_handler(entity, command, default_error_handler)
    }

    /// Queue a game command to be executed by the given entity,
    /// with a custom error handler.
    fn game_command_with_handler(
        &mut self,
        entity: Entity,
        command: impl AsRef<str>,
        error_handler: fn(BevyError, ErrorContext),
    ) -> &mut Self;
}

impl CommandsGameCommand for Commands<'_, '_> {
    #[inline]
    fn game_command_with_handler(
        &mut self,
        entity: Entity,
        command: impl AsRef<str>,
        error_handler: fn(BevyError, ErrorContext),
    ) -> &mut Self {
        self.entity(entity).game_command_with_handler(command, error_handler);
        self
    }
}

/// An extension trait for [`EntityCommands`] that adds
/// [`EntityCommands::game_command`].
pub trait EntityCommandsGameCommand {
    /// Queue a game command to be executed by the entity.
    #[inline]
    fn game_command(&mut self, command: impl AsRef<str>) -> &mut Self {
        self.game_command_with_handler(command, default_error_handler)
    }

    /// Queue a game command to be executed by the entity,
    /// with a custom error handler.
    fn game_command_with_handler(
        &mut self,
        command: impl AsRef<str>,
        error_handler: fn(BevyError, ErrorContext),
    ) -> &mut Self;
}

impl EntityCommandsGameCommand for EntityCommands<'_> {
    fn game_command_with_handler(
        &mut self,
        command: impl AsRef<str>,
        error_handler: fn(BevyError, ErrorContext),
    ) -> &mut Self {
        let command = String::from(command.as_ref());

        self.queue_handled(
            move |entity: EntityWorldMut| {
                let entity_id = entity.id();
                let world = entity.into_world_mut();

                // Split the command from it's arguments.
                let (command, arguments) =
                    command.trim_start().split_once(' ').unwrap_or((&command, ""));

                // Execute the command.
                let result = world.try_resource_scope::<GameCommandSet, _>(|world, commands| {
                    commands.execute(entity_id, command, arguments, world)
                });

                match result {
                    None | Some(Ok(())) => Ok(()),
                    Some(Err(err)) => {
                        Err(alloc::format!("Command \"{command}\" failed, {err}").into_boxed_str())
                    }
                }
            },
            error_handler,
        );
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// The default error handler for game commands.
#[allow(unused_variables, reason = "Used if \"tracing\" is enabled")]
pub fn default_error_handler(err: BevyError, ctx: ErrorContext) {
    #[cfg(feature = "tracing")]
    {
        if tracing::enabled!(target: "froglight_brigadier", tracing::Level::DEBUG) {
            tracing::error!(target: "froglight_brigadier", "{ctx}: {err}");
        } else {
            tracing::error!(target: "froglight_brigadier", "{err}");
        }
    }
}
