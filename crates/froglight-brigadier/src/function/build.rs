//! [`CommandBuilder`] and related types for building commands.

use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_reflect::{
    Reflect,
    func::{DynamicFunction, FunctionRegistry, IntoFunction},
};
use petgraph::graph::NodeIndex;
use smol_str::SmolStr;

use super::{Full, WorldRef};
use crate::graph::{BrigadierEdge, BrigadierError, BrigadierGraph, BrigadierNode};

/// A builder for adding commands to a [`BrigadierGraph`].
pub struct CommandBuilder<'env, State: BuilderState, Function> {
    pub(super) command: SmolStr,
    pub(super) previous: NodeIndex<u32>,
    pub(super) graph: &'env mut BrigadierGraph,
    pub(super) registry: &'env mut FunctionRegistry,
    pub(super) _phantom: PhantomData<(State, Function)>,
}

impl<'env> CommandBuilder<'env, Arg, fn(Entity, WorldRef<Full>)> {
    /// Create a new [`CommandBuilder`] for a [`BrigadierGraph`].
    ///
    /// # Errors
    /// Returns an error if the command already exists.
    pub(crate) fn try_new(
        command: impl Into<SmolStr>,
        graph: &'env mut BrigadierGraph,
        registry: &'env mut FunctionRegistry,
    ) -> Result<Self, BrigadierError> {
        let command = command.into();
        if graph.commands.contains_key(&command) {
            Err(BrigadierError::DuplicateCommand(command))
        } else {
            let entrypoint = graph.graph.add_node(BrigadierNode { function: None });
            graph.commands.insert(command.clone(), entrypoint);

            Ok(Self { command, graph, registry, previous: entrypoint, _phantom: PhantomData })
        }
    }
}

impl<'env, Function> CommandBuilder<'env, Arg, Function> {
    /// Build the command using the given function.
    ///
    /// # Errors
    /// Returns an error if the command could not be built or registered.
    #[expect(clippy::missing_panics_doc)]
    pub fn try_command<Marker>(
        mut self,
        f: Function,
    ) -> Result<CommandBuilder<'env, Command, Function>, BrigadierError>
    where
        Function: IntoFunction<'static, Marker>,
    {
        // Convert the function into a dynamic function.
        let mut dynamic = f.into_function();

        // Create and add the command node to the graph.
        let command = BrigadierNode { function: None };
        let current = self.graph.graph.add_node(command);
        self.graph.graph.add_edge(self.previous, current, BrigadierEdge::Command);

        // Update the command name to be unique.
        dynamic = Self::generate_function_name(dynamic, &self.command, current.index());
        self.graph.graph.node_weight_mut(current).unwrap().function = dynamic.name().cloned();

        // Register the function.
        self.registry.register(dynamic)?;

        // Update and return the builder.
        self.previous = current;
        Ok(self.convert())
    }

    fn generate_function_name<'a>(
        dynamic: DynamicFunction<'a>,
        command: &str,
        index: usize,
    ) -> DynamicFunction<'a> {
        // Generate a unique name for the function using the arguments.
        let signature = &dynamic.info().signatures()[0];
        let arguments = signature.args().iter().map(|arg| arg.type_path_table().short_path());
        let arguments = arguments.collect::<Vec<_>>().join("_");

        match dynamic.name().cloned() {
            Some(name) => dynamic.with_name(format!("brigadier_{index}_{name}_{arguments}")),
            None => dynamic.with_name(format!("brigadier_{index}_{command}_{arguments}")),
        }
    }
}

impl<'env, State: BuilderState, Function> CommandBuilder<'env, State, Function> {
    /// Add an empty [`BrigadierNode`] with the given [`BrigadierEdge`].
    ///
    /// This is equivalent to adding a parser to the command.
    #[inline]
    pub(crate) fn add_edge(&mut self, edge: BrigadierEdge) {
        self.add_node(BrigadierNode { function: None }, edge);
    }

    /// Add a [`BrigadierNode`] with the given [`BrigadierEdge`].
    pub(crate) fn add_node(&mut self, node: BrigadierNode, edge: BrigadierEdge) {
        // Add the new node to the graph.
        let current = self.graph.graph.add_node(node);
        // Add an edge connecting the previous node to the new node.
        self.graph.graph.add_edge(self.previous, current, edge);

        // Replace the previous node with the current node.
        self.previous = current;
    }

    /// Convert the function to a different type.
    ///
    /// # Note
    /// [`FunctionBuilder`] calls this internally,
    /// so this applies to all building functions.
    #[inline]
    #[must_use]
    pub(crate) fn convert<OtherState: BuilderState, OtherFunction>(
        self,
    ) -> CommandBuilder<'env, OtherState, OtherFunction> {
        CommandBuilder {
            command: self.command,
            previous: self.previous,
            graph: self.graph,
            registry: self.registry,
            _phantom: PhantomData,
        }
    }
}

/// A trait for describing the state of a [`CommandBuilder`].
pub trait BuilderState: Copy + Reflect + Send + Sync + 'static {}

/// A state of the [`CommandBuilder`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Arg;
impl BuilderState for Arg {}

/// A state of the [`CommandBuilder`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Command;
impl BuilderState for Command {}

/// A state of the [`CommandBuilder`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Fork;
impl BuilderState for Fork {}
