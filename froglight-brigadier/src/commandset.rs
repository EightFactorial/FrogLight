//! TODO

use alloc::{borrow::Cow, boxed::Box};
use core::{error::Error, fmt};

use bevy_ecs::{prelude::*, system::SystemId};
use bevy_reflect::{prelude::*, std_traits::ReflectDefault};
use foldhash::fast::RandomState;
use indexmap::IndexMap;

use crate::{argument::ArgumentParseError, bundle::ArgumentBundle, prelude::GameCommandCtx};

/// A set of commands that can be executed by entities.
#[derive(Default, Clone, Resource, Reflect)]
#[reflect(opaque, Default, Clone, Resource)]
pub struct GameCommandSet(IndexMap<Cow<'static, str>, CommandInfo, RandomState>);

impl GameCommandSet {
    /// Create a new empty [`GameCommandSet`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Register a command with the given name and system.
    ///
    /// # Errors
    ///
    /// Returns an error if a command with the same name already exists.
    #[inline]
    pub fn register_command<B: ArgumentBundle>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        system: SystemId<GameCommandCtx<B>, ()>,
    ) -> Result<(), CommandRegisterError>
    where
        B::BundleData: Default,
    {
        Self::register_command_using(self, command.into(), B::BundleData::default(), system)
    }

    /// Register a command with the given name, parser, and system.
    ///
    /// # Errors
    ///
    /// Returns an error if a command with the same name already exists.
    pub fn register_command_using<B: ArgumentBundle>(
        &mut self,
        command: Cow<'static, str>,
        settings: B::BundleData,
        system: SystemId<GameCommandCtx<B>, ()>,
    ) -> Result<(), CommandRegisterError> {
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_brigadier", "Registering \"{command}\" with {system:?}");

        if self.0.contains_key(&command) {
            Err(CommandRegisterError::AlreadyExists)
        } else {
            self.0.insert(command, CommandInfo::new::<B>(settings, system));
            Ok(())
        }
    }

    /// Execute a command with the given arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is not found or execution fails.
    pub fn execute<'a>(
        &self,
        entity: Entity,
        command: &'a str,
        arguments: &'a str,
        world: &mut World,
    ) -> Result<(), CommandExecuteError<'a>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_brigadier", "Entity {entity} executed command \"{command}\"");

        if let Some(info) = self.0.get(command) {
            info.run(entity, arguments, world)
        } else {
            Err(CommandExecuteError::CommandNotFound(Cow::Borrowed(command)))
        }
    }
}

/// An error that can occur while registering a command.
#[derive(Debug)]
pub enum CommandRegisterError {
    /// A command with the same name already exists.
    AlreadyExists,
}

impl Error for CommandRegisterError {}
impl fmt::Display for CommandRegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandRegisterError::AlreadyExists => {
                write!(f, "a command with the same name already exists")
            }
        }
    }
}

/// An error that can occur while parsing or executing a command.
#[derive(Debug)]
pub enum CommandExecuteError<'a> {
    /// The command was not found.
    CommandNotFound(Cow<'a, str>),
    /// An error occurred while parsing the command.
    Parse(ArgumentParseError<'a>),
    /// An error occurred while executing the command.
    Execute(Box<dyn Error + Send + Sync>),
}

impl CommandExecuteError<'_> {
    /// Take ownership of the error,
    /// converting any borrowed data into owned data.
    #[must_use]
    pub fn into_owned(self) -> CommandExecuteError<'static> {
        match self {
            CommandExecuteError::CommandNotFound(cmd) => {
                CommandExecuteError::CommandNotFound(Cow::Owned(cmd.into_owned()))
            }
            CommandExecuteError::Parse(err) => CommandExecuteError::Parse(err.into_owned()),
            CommandExecuteError::Execute(err) => CommandExecuteError::Execute(err),
        }
    }

    /// Create a new [`ParseOrExecuteError`] from an error.
    #[inline]
    #[must_use]
    pub fn execute<E: Error + Send + Sync + 'static>(err: E) -> Self {
        CommandExecuteError::Execute(Box::new(err))
    }
}

impl Error for CommandExecuteError<'_> {}
impl fmt::Display for CommandExecuteError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandExecuteError::CommandNotFound(cmd) => write!(f, "command \"{cmd}\" not found"),
            CommandExecuteError::Parse(err) => write!(f, "parsing error, {err}"),
            CommandExecuteError::Execute(err) => write!(f, "execution error, {err}"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Information about a command in the [`GameCommandSet`].
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity, reason = "dyn Fn trait")]
struct CommandInfo {
    runner: &'static (
                 dyn Fn(Entity, &str, &mut World) -> Result<(), CommandExecuteError<'static>>
                     + Send
                     + Sync
             ),
}

impl CommandInfo {
    /// Create a new [`CommandInfo`] for the given root node and system.
    #[must_use]
    fn new<B: ArgumentBundle>(
        data: B::BundleData,
        system: SystemId<GameCommandCtx<B>, ()>,
    ) -> Self {
        Self {
            runner: Box::leak(Box::new(
                move |entity: Entity, arguments: &str, world: &mut World| {
                    // Parse the `BundleData` from the command.
                    let input = B::bundle_from_string(arguments, &data)
                        .map_err(|err| CommandExecuteError::Parse(err.into_owned()))?;

                    // Run the system with the Entity and `BundleData` as input.
                    world
                        .run_system_with(system, (entity, input))
                        .map_err(CommandExecuteError::execute)
                },
            )),
        }
    }

    /// Run this command.
    ///
    /// # Errors
    ///
    /// Returns an error if execution fails.
    #[inline]
    fn run<'a>(
        &self,
        entity: Entity,
        arguments: &'a str,
        world: &mut World,
    ) -> Result<(), CommandExecuteError<'a>> {
        (self.runner)(entity, arguments, world)
    }
}

impl fmt::Debug for CommandInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandInfo").finish_non_exhaustive()
    }
}
