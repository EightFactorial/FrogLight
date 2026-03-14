//! TODO

use alloc::{borrow::Cow, boxed::Box};
use core::any::TypeId;

use bevy_ecs::prelude::*;
use bevy_reflect::{
    func::{ArgList, DynamicFunction, FunctionError},
    prelude::*,
};
use foldhash::fast::RandomState;
use indexmap::IndexMap;
use petgraph::prelude::*;

use crate::parse::{ArgumentParseError, CommandArgument, CommandArgumentDyn};

/// A graph of containing a tree of command nodes.
#[derive(Default, Clone, Resource, Reflect)]
#[reflect(opaque, Default, Clone, Resource)]
pub struct CommandGraph {
    commands:
        IndexMap<Cow<'static, str>, (NodeIndex, Option<DynamicFunction<'static>>), RandomState>,
    graph: StableDiGraph<CommandNode, CommandEdge>,
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

        let entrypoint = self.graph.add_node(CommandNode::Entrypoint);
        self.commands.insert(command, (entrypoint, function));
        Ok(entrypoint)
    }

    /// Register a parser for a command.
    ///
    /// # Errors
    ///
    /// Returns an error if no commands with the given name exist.
    pub fn register_parser(
        &mut self,
        command: impl AsRef<str>,
        edges: impl IntoIterator<Item = CommandEdge>,
    ) -> Result<(), ParserRegisterError> {
        if let Some((entrypoint, _)) = self.commands.get(command.as_ref()) {
            self.register_parser_from(*entrypoint, edges)
        } else {
            Err(ParserRegisterError::UnknownCommand)
        }
    }

    /// Register a parser starting at an entrypoint node.
    ///
    /// # Errors
    ///
    /// Returns an error if no command with the given entrypoint exists.
    pub fn register_parser_from(
        &mut self,
        entrypoint: NodeIndex,
        edges: impl IntoIterator<Item = CommandEdge>,
    ) -> Result<(), ParserRegisterError> {
        if self.graph.node_weight(entrypoint).copied() != Some(CommandNode::Entrypoint) {
            return Err(ParserRegisterError::InvalidEntrypoint);
        }

        let mut current_node = entrypoint;
        let mut edges = edges.into_iter().collect::<alloc::vec::Vec<_>>();
        edges.reverse();

        'outer: while let Some(current_edge) = edges.pop() {
            // If the edge already exists, use it.
            for edge in self.graph.edges(current_node) {
                if edge.weight() == &current_edge {
                    current_node = edge.target();
                    continue 'outer;
                }
            }

            // Otherwise, add a new node and edge.
            let new_node = self.graph.add_node(CommandNode::Argument);
            self.graph.add_edge(current_node, new_node, current_edge);
            current_node = new_node;
        }

        Ok(())
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
        let mut command = "";

        if let Some(pos) = command_name.find(' ') {
            (command_name, command) = command_name.split_at(pos);
        }

        let Some((entrypoint, _)) = self.commands.get(command_name) else {
            return Err(CommandExecuteError::UnknownCommand);
        };
        if self.graph.node_weight(*entrypoint).copied() != Some(CommandNode::Entrypoint) {
            return Err(CommandExecuteError::InvalidEntryPoint);
        }

        let mut current_node = *entrypoint;
        let mut arguments = ArgList::new();

        'outer: while !command.is_empty() {
            // First, try edges with conditions
            for edge in self.graph.edges(current_node) {
                if let Some(condition) = edge.weight().condition
                    && (condition)(&arguments, command)
                {
                    match edge.weight().parser.parse_value(command) {
                        // Push the argument and continue parsing
                        Ok((arg, rem)) => {
                            arguments.push_arg(arg);
                            command = rem.trim_start();
                            current_node = edge.target();
                            continue 'outer;
                        }
                        // If the input doesn't match, try the next edge.
                        Err(ArgumentParseError::InputMismatch) => {}
                        // Otherwise return the error.
                        Err(err) => return Err(CommandExecuteError::Parser(err)),
                    }
                }
            }

            // If no edges with conditions matched, try edges without conditions.
            for edge in self.graph.edges(current_node) {
                if edge.weight().condition.is_none() {
                    match edge.weight().parser.parse_value(command) {
                        // Push the argument and continue parsing
                        Ok((arg, rem)) => {
                            arguments.push_arg(arg);
                            command = rem.trim_start();
                            current_node = edge.target();
                            continue 'outer;
                        }
                        // If the input doesn't match, try the next edge.
                        Err(ArgumentParseError::InputMismatch) => {}
                        // Otherwise return the error.
                        Err(err) => return Err(CommandExecuteError::Parser(err)),
                    }
                }
            }

            // If no edges matched, the input is invalid.
            return Err(CommandExecuteError::Parser(ArgumentParseError::InputInvalid(
                command.into(),
            )));
        }

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
        let Some((_, function)) = self.commands.get(command.as_ref()) else {
            return Err(CommandExecuteError::UnknownCommand);
        };
        let Some(function) = function.as_ref() else {
            return Err(CommandExecuteError::MissingFunction);
        };

        // TODO: Push `&mut World` as the last argument
        // arguments.push_arg(world);

        function
            .call(arguments)
            .map_or_else(|err| Err(CommandExecuteError::Function(err)), |_| Ok(()))
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
    pub const fn as_graph(graph: &Self) -> &StableDiGraph<CommandNode, CommandEdge> { &graph.graph }

    /// Get a mutable reference to the inner command parser graph.
    #[inline]
    #[must_use]
    pub const fn as_graph_mut(graph: &mut Self) -> &mut StableDiGraph<CommandNode, CommandEdge> {
        &mut graph.graph
    }
}

// -------------------------------------------------------------------------------------------------

/// A node in the [`CommandGraph`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, Clone, PartialEq)]
pub enum CommandNode {
    /// A command entrypoint.
    Entrypoint,
    /// An argument node.
    Argument,
}

/// An edge in the [`CommandGraph`].
#[derive(Reflect)]
#[reflect(opaque, Clone, PartialEq)]
pub struct CommandEdge {
    /// An optional condition to traverse this edge.
    pub condition: Option<fn(&ArgList<'_>, &str) -> bool>,
    /// The parser for this edge.
    pub parser: Box<dyn CommandArgumentDyn>,
    /// The [`TypeId`] of the parser.
    pub parser_ty: TypeId,
    /// The [`TypeId`] of the parsed value.
    pub value_ty: TypeId,
}

impl Clone for CommandEdge {
    fn clone(&self) -> Self {
        Self {
            condition: self.condition,
            parser: self.parser.dyn_clone(),
            parser_ty: self.parser_ty,
            value_ty: self.value_ty,
        }
    }
}

impl Eq for CommandEdge {}
impl PartialEq for CommandEdge {
    fn eq(&self, other: &Self) -> bool {
        self.parser_ty == other.parser_ty
            && self.value_ty == other.value_ty
            && self.condition.is_some() == other.condition.is_some()
    }
}

impl CommandEdge {
    /// Create a new [`CommandEdge`] using the default [`CommandArgument`].
    #[inline]
    #[must_use]
    pub fn new<A: CommandArgument>() -> Self { Self::new_from::<A>(A::default()) }

    /// Create a new [`CommandEdge`] using the given [`CommandArgument`].
    #[must_use]
    pub fn new_from<A: CommandArgument>(argument: A) -> Self {
        Self {
            condition: None,
            parser: Box::new(argument),
            parser_ty: TypeId::of::<A>(),
            value_ty: TypeId::of::<A::Output>(),
        }
    }

    /// Create a new [`CommandEdge`] for the given [`CommandArgument`] and
    /// condition.
    #[must_use]
    pub fn new_optional<A: CommandArgument>(
        argument: A,
        condition: fn(&ArgList<'_>, &str) -> bool,
    ) -> Self {
        Self {
            condition: Some(condition),
            parser: Box::new(argument),
            parser_ty: TypeId::of::<A>(),
            value_ty: TypeId::of::<A::Output>(),
        }
    }
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
    /// The given node is not an entrypoint node.
    InvalidEntrypoint,
}

/// An error that can occur when executing a command.
#[derive(Debug)]
pub enum CommandExecuteError {
    /// The command does not exist.
    UnknownCommand,
    /// The command exists, but no function was registered for it.
    MissingFunction,
    /// The command exists, but the entrypoint node is invalid.
    InvalidEntryPoint,
    /// An error occurred while parsing command arguments.
    Parser(ArgumentParseError),
    /// An error occurred when calling the function.
    Function(FunctionError),
}
