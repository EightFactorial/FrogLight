//! TODO

use std::{any::TypeId, sync::Arc};

use bevy_ecs::prelude::*;
use bevy_reflect::{
    TypeRegistry,
    func::{ArgList, ArgValue, FunctionRegistry, FunctionResult},
    prelude::*,
};
use derive_more::Deref;
use hashbrown::HashMap;
use parking_lot::RwLock;
use petgraph::{Direction::Outgoing, graph::NodeIndex, prelude::StableDiGraph, visit::EdgeRef};
use smol_str::SmolStr;

use crate::argument::{ArgumentError, ReflectArgumentParser};

mod component;
use component::{BridagierEdge, BridagierNode};

mod error;
pub use error::BrigadierError;

/// A thread-safe brigadier-style command graph.
#[derive(Debug, Default, Clone, Resource, Deref, Reflect)]
#[reflect(opaque, Debug, Default, Resource)]
pub struct AppBrigadierGraph(Arc<RwLock<BrigadierGraph>>);

/// A brigadier-style command graph.
#[derive(Debug, Default, Clone)]
pub struct BrigadierGraph {
    commands: HashMap<SmolStr, NodeIndex<u32>>,
    graph: StableDiGraph<BridagierNode, BridagierEdge>,
}

impl BrigadierGraph {
    /// Parse and execute a command.
    ///
    /// # Errors
    /// Returns an error if the command was unknown,
    /// or if an argument was invalid.
    pub fn execute(
        &self,
        command: impl AsRef<str>,
        registry: &TypeRegistry,
        functions: &FunctionRegistry,
    ) -> Result<(), BrigadierError> {
        let (node, args) = self.build_command(command.as_ref(), registry)?;
        if let Some(function) = node.function.as_ref() {
            match functions.call(function.as_str(), args) {
                Some(FunctionResult::Ok(_)) => Ok(()),
                Some(FunctionResult::Err(err)) => Err(BrigadierError::Function(err)),
                None => Err(BrigadierError::UnknownCommand(function.clone())),
            }
        } else {
            // Shouldn't happen, but just in case.
            Err(BrigadierError::UnexpectedEnd(SmolStr::from(command.as_ref())))
        }
    }

    /// Attempt to parse a command.
    ///
    /// Similar to [`BridagierGraph::execute`],
    /// but does not actually execute the command.
    ///
    /// # Errors
    /// Returns an error if the command was unknown,
    /// or if an argument was invalid.
    pub fn parse(
        &self,
        command: impl AsRef<str>,
        registry: &TypeRegistry,
        functions: &FunctionRegistry,
    ) -> Result<(), BrigadierError> {
        let (node, _) = self.build_command(command.as_ref(), registry)?;
        if let Some(function) = node.function.as_ref() {
            if functions.contains(function.as_str()) {
                Ok(())
            } else {
                Err(BrigadierError::UnknownFunction(function.clone()))
            }
        } else {
            // Shouldn't happen, but just in case.
            Err(BrigadierError::UnexpectedEnd(SmolStr::from(command.as_ref())))
        }
    }

    /// Build a function call from a string and [`TypeRegistry`].
    ///
    /// # Errors
    /// Returns an error if the command was unknown,
    /// or if an argument was invalid.
    fn build_command<'a>(
        &'a self,
        mut command: &'a str,
        registry: &TypeRegistry,
    ) -> Result<(&'a BridagierNode, ArgList<'a>), BrigadierError> {
        command = command.trim();

        let (com, mut arg) = match command.split_once(' ') {
            Some((com, arg)) => (com, arg),
            None => (command, ""),
        };

        // Get the starting command node.
        let Some(mut index) = self.commands.get(com).copied() else {
            return Err(BrigadierError::UnknownCommand(com.into()));
        };

        // Parse the command and build the argument list.
        let mut list = ArgList::new();
        'node: while let Some(node) = self.graph.node_weight(index) {
            // Stop if the argument string is empty.
            if arg.is_empty() {
                // No more arguments, but executable.
                if node.function.is_some() {
                    return Ok((node, list));
                }

                // No more arguments, but not executable.
                return Err(BrigadierError::UnexpectedEnd(SmolStr::from(command)));
            }

            for edge in self.graph.edges_directed(index, Outgoing) {
                match edge.weight() {
                    BridagierEdge::Literal(str) => {
                        if let Some(remaining) = Self::parse_literal(arg, str) {
                            // Update the remaining arguments.
                            arg = remaining;

                            // Move to the next node.
                            index = edge.target();
                            continue 'node;
                        }
                    }
                    BridagierEdge::Argument(type_id) => {
                        let (argument, remaining) = Self::parse_argument(arg, *type_id, registry)?;

                        // If an argument was parsed, add it to the list.
                        if let Some(argument) = argument {
                            list = list.push_arg(argument);
                        }

                        // Update the remaining argument string.
                        arg = remaining;

                        // Move to the next node.
                        index = edge.target();
                        continue 'node;
                    }
                }
            }

            break;
        }

        // Unable to find a node, return an error.
        Err(BrigadierError::Argument(ArgumentError::InvalidArgument(
            command.len() - (com.len() + arg.len()),
        )))
    }

    fn parse_literal<'a>(arg: &'a str, literal: &str) -> Option<&'a str> {
        if let Some(stripped) = arg.strip_prefix(literal) { Some(stripped) } else { None }
    }

    #[expect(clippy::match_wildcard_for_single_variants)]
    fn parse_argument<'a>(
        arguments: &'a str,
        type_id: TypeId,
        registry: &TypeRegistry,
    ) -> Result<(Option<ArgValue<'a>>, &'a str), BrigadierError> {
        // Immediately return an error if the parser is unknown.
        let Some(parser_type) = registry.get(type_id) else {
            return Err(BrigadierError::UnknownParser(None));
        };

        // Immediately return an error if the parser has no data.
        let Some(parser_type) = registry.get(type_id) else {
            return Err(BrigadierError::UnknownParser(Some(
                parser_type.type_info().type_path_table().short_path(),
            )));
        };

        // Attempt to parse the argument.
        if let Some(parser) = parser_type.data::<ReflectArgumentParser>() {
            parser.parse(arguments).map_or_else(
                |err| match err {
                    // Ignore when the argument does not match
                    ArgumentError::DoesNotMatch => Ok((None, arguments)),
                    // Propagate other errors
                    _ => Err(BrigadierError::Argument(err)),
                },
                |(value, remaining)| Ok((Some(value), remaining)),
            )
        } else {
            Ok((None, arguments))
        }
    }
}
