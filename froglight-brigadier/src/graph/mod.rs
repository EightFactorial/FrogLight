//! TODO

use alloc::borrow::Cow;

use bevy_ecs::prelude::*;
use bevy_reflect::{
    func::{ArgList, DynamicFunction, FunctionError},
    prelude::*,
};
use foldhash::fast::RandomState;
use indexmap::IndexMap;
use petgraph::prelude::*;

use crate::builder::ArgumentBundle;

/// A graph of containing a tree of command nodes.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(opaque, Debug, Default, Clone, Resource)]
pub struct CommandGraph {
    commands:
        IndexMap<Cow<'static, str>, (NodeIndex, Option<DynamicFunction<'static>>), RandomState>,
    graph: StableDiGraph<(), ()>,
}

impl CommandGraph {
    /// Create a new, empty [`CommandGraph`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Register a command with the [`CommandGraph`].
    ///
    /// # Errors
    ///
    /// Returns an error if a command with the same name already exists.
    pub fn register_function(
        &mut self,
        command: impl Into<Cow<'static, str>>,
        function: Option<DynamicFunction<'static>>,
    ) -> Result<NodeIndex, CommandRegisterError> {
        let command = command.into();
        if self.commands.contains_key(&command) {
            return Err(CommandRegisterError::AlreadyExists);
        }

        let entrypoint = self.graph.add_node(());
        self.commands.insert(command, (entrypoint, function));
        Ok(entrypoint)
    }

    /// Register a parser for a command.
    ///
    /// # Errors
    ///
    /// Returns an error if no commands with the given name exist.
    pub fn register_parser<A: ArgumentBundle>(
        &mut self,
        command: impl AsRef<str>,
    ) -> Result<(), ParserRegisterError> {
        if let Some((entrypoint, _)) = self.commands.get(command.as_ref()) {
            self.register_parser_from::<A>(*entrypoint)
        } else {
            Err(ParserRegisterError::UnknownCommand)
        }
    }

    /// Register a parser starting at an entrypoint node.
    ///
    /// # Errors
    ///
    /// Returns an error if no command with the given entrypoint exists.
    pub fn register_parser_from<A: ArgumentBundle>(
        &mut self,
        _entrypoint: NodeIndex,
    ) -> Result<(), ParserRegisterError> {
        todo!("Create parser from `ArgumentBundle`");
    }

    /// Run a command from the [`CommandGraph`].
    ///
    /// # Note
    ///
    /// `command` should be the full, unparsed command without a leading slash.
    ///
    /// # Errors
    ///
    /// Returns an error if the command does not exist or the arguments are
    /// invalid.
    pub fn parse_and_run_command(
        &self,
        command: impl AsRef<str>,
        world: &mut World,
    ) -> Result<(), CommandExecuteError> {
        let mut command_name = command.as_ref().trim();
        let mut _command = "";

        if let Some(pos) = command_name.find(' ') {
            (command_name, _command) = command_name.split_at(pos);
        }

        let arguments = ArgList::new();
        // TODO: Use the graph to populate `arguments`

        self.run_command(command_name, arguments, world)
    }

    /// Run a command from the [`CommandGraph`].
    ///
    /// # Note
    ///
    /// `command` should be the command name without a leading slash.
    ///
    /// # Errors
    ///
    /// Returns an error if the command does not exist or the arguments are
    /// invalid.
    pub fn run_command(
        &self,
        command: impl AsRef<str>,
        arguments: ArgList<'_>,
        _world: &mut World,
    ) -> Result<(), CommandExecuteError> {
        if let Some((_, function)) = self.commands.get(command.as_ref()) {
            let Some(function) = function.as_ref() else {
                return Err(CommandExecuteError::MissingFunction);
            };

            // TODO: Push `&mut World` as the last argument
            // arguments.push_arg(world);

            function
                .call(arguments)
                .map_or_else(|err| Err(CommandExecuteError::Function(err)), |_| Ok(()))
        } else {
            Err(CommandExecuteError::UnknownCommand)
        }
    }

    /// Get a reference to a command's [`DynamicFunction`].
    #[must_use]
    pub fn command(&self, command: impl AsRef<str>) -> Option<&DynamicFunction<'static>> {
        self.commands.get(command.as_ref()).and_then(|(_, func)| func.as_ref())
    }

    /// Get a reference to the inner command map.
    #[inline]
    #[must_use]
    pub const fn as_commands(
        graph: &Self,
    ) -> &IndexMap<Cow<'static, str>, (NodeIndex, Option<DynamicFunction<'static>>), RandomState>
    {
        &graph.commands
    }

    /// Get a mutable reference to the inner command map.
    #[inline]
    #[must_use]
    pub const fn as_commands_mut(
        graph: &mut Self,
    ) -> &mut IndexMap<Cow<'static, str>, (NodeIndex, Option<DynamicFunction<'static>>), RandomState>
    {
        &mut graph.commands
    }

    /// Get a reference to the inner command parser graph.
    #[inline]
    #[must_use]
    pub const fn as_graph(graph: &Self) -> &StableDiGraph<(), ()> { &graph.graph }

    /// Get a mutable reference to the inner command parser graph.
    #[inline]
    #[must_use]
    pub const fn as_graph_mut(graph: &mut Self) -> &mut StableDiGraph<(), ()> { &mut graph.graph }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when registering a command.
#[derive(Debug)]
pub enum CommandRegisterError {
    /// A command with the same name already exists.
    AlreadyExists,
}

/// An error that can occur when registering a command parser.
#[derive(Debug)]
pub enum ParserRegisterError {
    /// No command with the given name exists.
    UnknownCommand,
}

/// An error that can occur when executing a command.
#[derive(Debug)]
pub enum CommandExecuteError {
    /// The command does not exist.
    UnknownCommand,
    /// The command exists, but no function was registered for it.
    MissingFunction,
    /// An error occurred when calling the function.
    Function(FunctionError),
}
