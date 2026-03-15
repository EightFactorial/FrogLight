//! TODO

use alloc::{borrow::Cow, boxed::Box, string::String};
use core::error::Error;

use bevy_ecs::{prelude::*, system::SystemId};
use bevy_reflect::{func::ArgList, prelude::*, std_traits::ReflectDefault};
use foldhash::fast::RandomState;
use indexmap::IndexMap;
use petgraph::prelude::*;

/// A set of commands with parsing logic represented as a graph.
#[derive(Default, Clone, Reflect, Resource)]
#[reflect(opaque, Default, Clone, Resource)]
pub struct CommandGraph {
    commands: IndexMap<Cow<'static, str>, CommandInfo, RandomState>,
    #[expect(unused, reason = "WIP")]
    graph: StableDiGraph<(), CommandEdge>,
}

impl CommandGraph {
    /// Create a new empty [`CommandGraph`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Parse a command string into an [`ArgList`].
    ///
    /// # Errors
    ///
    /// Returns an error if the command is not found or parsing fails.
    pub fn parse<'a>(&self, mut command: &'a str) -> Result<ArgList<'a>, CommandParseError> {
        // Separate the command name from the arguments.
        let mut arguments = "";
        if let Some((name, args)) = command.split_once(' ') {
            (command, arguments) = (name, args);
        }

        if let Some(_info) = self.commands.get(command) {
            todo!("Parse arguments: \"{arguments}\"");
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
        command: &str,
        args: ArgList<'_>,
        world: &mut World,
    ) -> Result<(), CommandExecuteError> {
        if let Some(info) = self.commands.get(command) {
            info.run(args, world).map_err(CommandExecuteError::CommandError)
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
        mut command: &str,
        world: &mut World,
    ) -> Result<(), ParseOrExecuteError> {
        let args = self.parse(command).map_err(ParseOrExecuteError::Parse)?;
        // Extract the command name to execute, ignoring the arguments.
        if let Some((name, _)) = command.split_once(' ') {
            command = name;
        }
        self.execute(command, args, world).map_err(ParseOrExecuteError::Execute)
    }
}

/// An error that can occur while parsing a command string.
#[derive(Debug)]
pub enum CommandParseError {
    /// The command was not found.
    CommandNotFound(String),
    /// An error occurred while parsing the command.
    ParseError(Box<dyn Error + Send + Sync>),
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

/// Information about a command in the [`CommandGraph`].
#[derive(Reflect)]
#[reflect(opaque)]
pub struct CommandInfo {
    root: NodeIndex,
    runner: Box<dyn GraphFn>,
}

impl CommandInfo {
    /// Create a new [`CommandInfo`] for the given root node and system.
    #[must_use]
    #[allow(unused, reason = "WIP")]
    pub fn new<I: 'static>(root: NodeIndex, system: SystemId<In<I>, ()>) -> Self {
        Self {
            root,
            #[expect(unused, reason = "WIP")]
            runner: Box::new(move |args, world| {
                todo!("ArgList -> In<I>");
                world
                    .run_system_with(system, todo!())
                    .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
            }),
        }
    }

    /// Get the [`NodeIndex`] for the root node of this command.
    #[inline]
    #[must_use]
    pub const fn root(&self) -> NodeIndex { self.root }

    /// Run this command.
    ///
    /// # Errors
    ///
    /// Returns an error if execution fails.
    #[inline]
    pub fn run(
        &self,
        args: ArgList<'_>,
        world: &mut World,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        (self.runner)(args, world)
    }
}

impl Eq for CommandInfo {}
impl PartialEq for CommandInfo {
    fn eq(&self, other: &Self) -> bool { self.root == other.root }
}

impl Clone for CommandInfo {
    fn clone(&self) -> Self { Self { root: self.root, runner: self.runner.dyn_clone() } }
}

// -------------------------------------------------------------------------------------------------

/// A trait for functions that can be stored in the [`CommandGraph`].
pub trait GraphFn:
    Fn(ArgList<'_>, &mut World) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync + 'static
{
    /// Clone this function as a trait object.
    fn dyn_clone(&self) -> Box<dyn GraphFn>;
}
impl<T> GraphFn for T
where
    T: Clone
        + Fn(ArgList<'_>, &mut World) -> Result<(), Box<dyn Error + Send + Sync>>
        + Send
        + Sync
        + 'static,
{
    fn dyn_clone(&self) -> Box<dyn GraphFn> { Box::new(self.clone()) }
}

// -------------------------------------------------------------------------------------------------

/// An edge in the [`CommandGraph`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandEdge {}
