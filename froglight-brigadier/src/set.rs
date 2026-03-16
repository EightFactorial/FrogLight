//! TODO

use alloc::{borrow::Cow, boxed::Box, string::String};
use core::{any::TypeId, error::Error};

use bevy_ecs::{prelude::*, system::SystemId};
use bevy_reflect::{func::ArgList, prelude::*, std_traits::ReflectDefault};
use foldhash::fast::RandomState;
use indexmap::IndexMap;

use crate::{
    argument::ArgumentParseError,
    bundle::{ArgumentParserBundle, ArgumentParserBundleExt},
};

/// A set of commands that can be executed by entities.
#[derive(Default, Clone, Reflect, Resource)]
#[reflect(opaque, Default, Clone, Resource)]
pub struct CommandSet(IndexMap<Cow<'static, str>, CommandInfo, RandomState>);

impl CommandSet {
    /// Create a new empty [`CommandSet`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Register a command with the given name and system.
    ///
    /// # Errors
    ///
    /// Returns an error if a command with the same name already exists.
    #[inline]
    pub fn register_command<B: Default + ArgumentParserBundleExt>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        system: SystemId<In<CommandCtx<B::Arguments>>, ()>,
    ) -> Result<(), CommandRegisterError> {
        Self::register_command_using(self, command, B::default(), system)
    }

    /// Register a command with the given name, parser, and system.
    ///
    /// # Errors
    ///
    /// Returns an error if a command with the same name already exists.
    pub fn register_command_using<B: ArgumentParserBundleExt>(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        parser: B,
        system: SystemId<In<CommandCtx<B::Arguments>>, ()>,
    ) -> Result<(), CommandRegisterError> {
        let command = command.into();
        if self.0.contains_key(&command) {
            Err(CommandRegisterError::AlreadyExists)
        } else {
            self.0.insert(command, CommandInfo::new_from::<B>(parser, system));
            Ok(())
        }
    }

    /// Parse a command string into an [`ArgList`].
    ///
    /// # Errors
    ///
    /// Returns an error if the command is not found or parsing fails.
    pub fn parse(&self, mut command: &str) -> Result<ArgList<'static>, CommandParseError> {
        // Separate the command name from the arguments.
        let mut arguments = "";
        if let Some((name, args)) = command.split_once(' ') {
            (command, arguments) = (name, args);
        }

        if let Some(info) = self.0.get(command) {
            info.parser.try_from_string(arguments).map_err(CommandParseError::ParseError)
        } else {
            Err(CommandParseError::CommandNotFound(command.into()))
        }
    }

    /// Execute a command with the given arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is not found or execution fails.
    pub fn execute(
        &self,
        entity: Entity,
        command: &str,
        args: ArgList<'static>,
        world: &mut World,
    ) -> Result<(), CommandExecuteError> {
        if let Some(info) = self.0.get(command) {
            info.run(entity, args, world).map_err(CommandExecuteError::CommandError)
        } else {
            Err(CommandExecuteError::CommandNotFound(command.into()))
        }
    }

    /// Parse and execute a command string.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is not found,
    /// or fails to parse and execute.
    pub fn parse_and_execute(
        &self,
        entity: Entity,
        mut command: &str,
        world: &mut World,
    ) -> Result<(), ParseOrExecuteError> {
        let args = self.parse(command).map_err(ParseOrExecuteError::Parse)?;
        // Extract the command name to execute, ignoring the arguments.
        if let Some((name, _)) = command.split_once(' ') {
            command = name;
        }
        self.execute(entity, command, args, world).map_err(ParseOrExecuteError::Execute)
    }
}

/// An error that can occur while registering a command.
#[derive(Debug)]
pub enum CommandRegisterError {
    /// A command with the same name already exists.
    AlreadyExists,
}

/// An error that can occur while parsing a command string.
#[derive(Debug)]
pub enum CommandParseError {
    /// The command was not found.
    CommandNotFound(String),
    /// An error occurred while parsing the command.
    ParseError(ArgumentParseError),
}

/// An error that can occur while executing a command.
#[derive(Debug)]
pub enum CommandExecuteError {
    /// The command was not found.
    CommandNotFound(String),
    /// An error occurred while executing the command.
    CommandError(Box<dyn Error + Send + Sync>),
}

/// An error that can occur while parsing or executing a command.
#[derive(Debug)]
pub enum ParseOrExecuteError {
    /// An error occurred while parsing the command.
    Parse(CommandParseError),
    /// An error occurred while executing the command.
    Execute(CommandExecuteError),
}

// -------------------------------------------------------------------------------------------------

/// Information about a command in the [`CommandSet`].
#[derive(Reflect)]
#[reflect(opaque)]
pub struct CommandInfo {
    parser_ty: TypeId,
    parser: Box<dyn ArgumentParserBundle>,
    runner: Box<dyn SetFn>,
}

impl CommandInfo {
    /// Create a new [`CommandInfo`] for the given root node and system.
    #[inline]
    #[must_use]
    pub fn new<B: Default + ArgumentParserBundleExt>(
        system: SystemId<In<CommandCtx<B::Arguments>>, ()>,
    ) -> Self {
        Self::new_from(B::default(), system)
    }

    /// Create a new [`CommandInfo`] for the given root node and system.
    #[must_use]
    pub fn new_from<B: ArgumentParserBundleExt>(
        parser: B,
        system: SystemId<In<CommandCtx<B::Arguments>>, ()>,
    ) -> Self {
        Self {
            parser_ty: TypeId::of::<B>(),
            parser: Box::new(parser),
            runner: Box::new(move |entity, args, world| {
                world
                    .run_system_with(system, CommandCtx::new(entity, B::try_from_args(args)?))
                    .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
            }),
        }
    }

    /// Run this command.
    ///
    /// # Errors
    ///
    /// Returns an error if execution fails.
    #[inline]
    pub fn run(
        &self,
        entity: Entity,
        args: ArgList<'static>,
        world: &mut World,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        (self.runner)(entity, args, world)
    }
}

impl Eq for CommandInfo {}
impl PartialEq for CommandInfo {
    fn eq(&self, other: &Self) -> bool { self.parser_ty == other.parser_ty }
}

impl Clone for CommandInfo {
    fn clone(&self) -> Self {
        Self {
            parser_ty: self.parser_ty,
            parser: self.parser.dyn_clone(),
            runner: self.runner.dyn_clone(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Context provided to command systems.
pub struct CommandCtx<T> {
    entity: Entity,
    input: T,
}

impl<T> CommandCtx<T> {
    /// Create a new [`CommandCtx`] with the given entity and input.
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity, input: T) -> Self { Self { entity, input } }

    /// Get the [`Entity`] that is executing this command.
    #[inline]
    #[must_use]
    pub const fn entity(&self) -> Entity { self.entity }

    /// Get a reference to the input for this command.
    #[inline]
    #[must_use]
    pub const fn input(&self) -> &T { &self.input }

    /// Get the input for this command.
    #[inline]
    #[must_use]
    pub fn into_input(self) -> T { self.input }
}

// -------------------------------------------------------------------------------------------------

/// A trait for functions that can be stored in the [`CommandSet`].
pub trait SetFn:
    Fn(Entity, ArgList<'static>, &mut World) -> Result<(), Box<dyn Error + Send + Sync>>
    + Send
    + Sync
    + 'static
{
    /// Clone this function as a trait object.
    fn dyn_clone(&self) -> Box<dyn SetFn>;
}
impl<T> SetFn for T
where
    T: Clone
        + Fn(Entity, ArgList<'static>, &mut World) -> Result<(), Box<dyn Error + Send + Sync>>
        + Send
        + Sync
        + 'static,
{
    fn dyn_clone(&self) -> Box<dyn SetFn> { Box::new(self.clone()) }
}
