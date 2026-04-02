//! TODO

use alloc::{borrow::Cow, boxed::Box};
use core::{
    any::TypeId,
    error::Error,
    fmt::{self, Display},
};

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
        world: &mut World,
    ) -> Result<(), ParseOrExecuteError<'a>> {
        if let Some(info) = self.0.get(command) {
            info.run(entity, command, world)
        } else {
            Err(ParseOrExecuteError::Parse(CommandParseError::CommandNotFound(command.into())))
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
impl Display for CommandRegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandRegisterError::AlreadyExists => {
                write!(f, "a command with the same name already exists")
            }
        }
    }
}

/// An error that can occur while parsing a command string.
#[derive(Debug)]
pub enum CommandParseError<'a> {
    /// The command was not found.
    CommandNotFound(Cow<'a, str>),
    /// An error occurred while parsing the command.
    ParseError(ArgumentParseError),
}

impl CommandParseError<'_> {
    /// Take ownership of the error, converting any borrowed data into owned
    /// data.
    #[must_use]
    pub fn into_owned(self) -> CommandParseError<'static> {
        match self {
            CommandParseError::CommandNotFound(cmd) => {
                CommandParseError::CommandNotFound(cmd.into_owned().into())
            }
            CommandParseError::ParseError(err) => CommandParseError::ParseError(err),
        }
    }
}

impl Error for CommandParseError<'_> {}
impl Display for CommandParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandParseError::CommandNotFound(cmd) => write!(f, "command not found: \"{cmd}\""),
            CommandParseError::ParseError(err) => Display::fmt(err, f),
        }
    }
}

/// An error that can occur while executing a command.
#[derive(Debug)]
pub enum CommandExecuteError {
    /// An error occurred while executing the command.
    CommandError(Box<dyn Error + Send + Sync>),
}

impl Error for CommandExecuteError {}
impl Display for CommandExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandExecuteError::CommandError(err) => Display::fmt(err, f),
        }
    }
}

/// An error that can occur while parsing or executing a command.
#[derive(Debug)]
pub enum ParseOrExecuteError<'a> {
    /// An error occurred while parsing the command.
    Parse(CommandParseError<'a>),
    /// An error occurred while executing the command.
    Execute(CommandExecuteError),
}

impl ParseOrExecuteError<'_> {
    /// Take ownership of the error, converting any borrowed data into owned
    /// data.
    #[must_use]
    pub fn into_owned(self) -> ParseOrExecuteError<'static> {
        match self {
            ParseOrExecuteError::Parse(err) => ParseOrExecuteError::Parse(err.into_owned()),
            ParseOrExecuteError::Execute(err) => ParseOrExecuteError::Execute(err),
        }
    }
}

impl Error for ParseOrExecuteError<'_> {}
impl Display for ParseOrExecuteError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseOrExecuteError::Parse(err) => write!(f, "Parse error, {err}"),
            ParseOrExecuteError::Execute(err) => write!(f, "Execute error: {err}"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Information about a command in the [`GameCommandSet`].
#[derive(Reflect)]
#[reflect(opaque)]
pub struct CommandInfo {
    parser_ty: TypeId,
    runner: Box<dyn CommandSetFn>,
}

impl CommandInfo {
    /// Create a new [`CommandInfo`] for the given root node and system.
    #[must_use]
    pub fn new<B: ArgumentBundle>(
        data: B::BundleData,
        system: SystemId<GameCommandCtx<B>, ()>,
    ) -> Self {
        Self {
            parser_ty: TypeId::of::<B>(),
            runner: Box::new(move |entity, args, world| {
                // Parse the `BundleData` from the input string.
                let input = B::bundle_from_string(args, &data).map_err(|err| {
                    ParseOrExecuteError::Parse(CommandParseError::ParseError(err))
                })?;
                // Run the system with the Entity and `BundleData` as input.
                world.run_system_with(system, (entity, input)).map_err(|err| {
                    ParseOrExecuteError::Execute(CommandExecuteError::CommandError(Box::new(err)))
                })
            }),
        }
    }

    /// Run this command.
    ///
    /// # Errors
    ///
    /// Returns an error if execution fails.
    #[inline]
    pub fn run<'a>(
        &self,
        entity: Entity,
        input: &'a str,
        world: &mut World,
    ) -> Result<(), ParseOrExecuteError<'a>> {
        (self.runner)(entity, input, world)
    }
}

impl Eq for CommandInfo {}
impl PartialEq for CommandInfo {
    fn eq(&self, other: &Self) -> bool { self.parser_ty == other.parser_ty }
}

impl Clone for CommandInfo {
    fn clone(&self) -> Self { Self { parser_ty: self.parser_ty, runner: self.runner.dyn_clone() } }
}

// -------------------------------------------------------------------------------------------------

/// A trait for functions that can be stored in the [`GameCommandSet`].
pub trait CommandSetFn:
    for<'a> Fn(Entity, &'a str, &mut World) -> Result<(), ParseOrExecuteError<'a>>
    + Send
    + Sync
    + 'static
{
    /// Clone this function as a trait object.
    fn dyn_clone(&self) -> Box<dyn CommandSetFn>;
}
impl<T> CommandSetFn for T
where
    T: for<'a> Fn(Entity, &'a str, &mut World) -> Result<(), ParseOrExecuteError<'a>>
        + Clone
        + Send
        + Sync
        + 'static,
{
    fn dyn_clone(&self) -> Box<dyn CommandSetFn> { Box::new(self.clone()) }
}
