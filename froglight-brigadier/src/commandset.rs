//! TODO

use alloc::{borrow::Cow, boxed::Box, sync::Arc};
use core::{any::TypeId, error::Error, fmt};

use bevy_ecs::{prelude::*, system::SystemId};
use bevy_reflect::{prelude::*, std_traits::ReflectDefault};
use froglight_common::types::IndexMap;

use crate::{
    argument::ArgumentParseError,
    bundle::{ArgumentBundle, BundleWrapper, ExecutableParser},
    prelude::GameCommandCtx,
};

/// A set of commands that can be executed by entities.
#[derive(Default, Clone, Resource, Reflect)]
#[reflect(opaque, Default, Clone, Resource)]
pub struct GameCommandSet(IndexMap<Cow<'static, str>, CommandInfo>);

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
#[derive(Clone)]
#[allow(dead_code, reason = "May be used in the future")]
struct CommandInfo {
    system_id: Entity,
    bundle_type: TypeId,
    executor: Arc<dyn ExecutableParser>,
}

impl CommandInfo {
    /// Create a new [`CommandInfo`] for the given root node and system.
    #[inline]
    #[must_use]
    fn new<B: ArgumentBundle>(
        settings: B::BundleData,
        system_id: SystemId<GameCommandCtx<B>, ()>,
    ) -> Self {
        Self {
            system_id: system_id.entity(),
            bundle_type: TypeId::of::<B>(),
            executor: BundleWrapper::<B>::new_executor(settings),
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
        caller: Entity,
        arguments: &'a str,
        world: &mut World,
    ) -> Result<(), CommandExecuteError<'a>> {
        self.executor.system_runner(caller, arguments, self.system_id, world)
    }
}

impl fmt::Debug for CommandInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandInfo").finish_non_exhaustive()
    }
}
