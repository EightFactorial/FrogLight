//! TODO

use alloc::borrow::Cow;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{
    bundle::ArgumentBundle,
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
    #[inline]
    fn add_game_command<B: ArgumentBundle, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        system: impl IntoSystem<CommandCtx<B>, (), Marker> + 'static,
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
        system: impl IntoSystem<CommandCtx<B>, (), Marker> + 'static,
    );
}

impl AppGameCommand for App {
    fn add_game_command_using<B: ArgumentBundle, Marker>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        settings: B::BundleData,
        system: impl IntoSystem<CommandCtx<B>, (), Marker> + 'static,
    ) {
        let system = self.world_mut().register_system_cached(system);
        if let Err(err) = self
            .world_mut()
            .get_resource_or_init::<CommandSet>()
            .register_command_using(command, settings, system)
        {
            panic!("Failed to register command: {err:?}");
        }
    }
}
