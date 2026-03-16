//! TODO

use alloc::borrow::Cow;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{
    bundle::ArgumentParserBundleExt,
    prelude::{CommandCtx, CommandSet},
};

/// A extension trait adding [`App::add_game_command`] and
/// [`App::add_game_command_using`].
pub trait AppGameCommand {
    /// Add a game command the the [`App`].
    ///
    /// # Panics
    ///
    /// Panics if a command with the same name is already registered.
    fn add_game_command<B: Default + ArgumentParserBundleExt, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        system: impl IntoSystem<In<CommandCtx<B::Arguments>>, (), Marker> + 'static,
    ) {
        self.add_game_command_using(command, B::default(), system);
    }

    /// Add a game command the the [`App`].
    ///
    /// # Panics
    ///
    /// Panics if a command with the same name is already registered.
    fn add_game_command_using<B: ArgumentParserBundleExt, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        parser: B,
        system: impl IntoSystem<In<CommandCtx<B::Arguments>>, (), Marker> + 'static,
    );
}

impl AppGameCommand for App {
    fn add_game_command_using<B: ArgumentParserBundleExt, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        parser: B,
        system: impl IntoSystem<In<CommandCtx<B::Arguments>>, (), Marker> + 'static,
    ) {
        let system = self.world_mut().register_system_cached(system);
        let command = command.into();

        if let Err(err) = self
            .world_mut()
            .get_resource_or_init::<CommandSet>()
            .register_command_using(command, parser, system)
        {
            panic!("Failed to register command: {err:?}");
        }
    }
}
