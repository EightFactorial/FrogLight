//! TODO

use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_reflect::func::{FunctionRegistry, IntoFunction};
use petgraph::graph::NodeIndex;
use smol_str::SmolStr;

mod build;
pub use build::FunctionBuilder;

#[cfg(test)]
mod test;

mod world;
pub use world::{Empty, Full, WorldRef};

use crate::graph::{BrigadierEdge, BrigadierError, BrigadierGraph, BrigadierNode};

/// A builder for adding commands to a [`BrigadierGraph`].
pub struct CommandBuilder<'env, Function> {
    command: SmolStr,
    entrypoint: Option<NodeIndex<u32>>,
    previous: Option<(NodeIndex<u32>, BrigadierEdge)>,
    graph: &'env mut BrigadierGraph,
    registry: &'env mut FunctionRegistry,
    _function: PhantomData<Function>,
}

#[allow(dead_code)]
impl<'env> CommandBuilder<'env, fn(Entity, WorldRef<Full>)> {
    /// Create a new [`CommandBuilder`] for a [`BrigadierGraph`].
    ///
    /// # Panics
    /// Panics if the command already exists.
    #[must_use]
    pub(crate) fn new(
        command: impl Into<SmolStr>,
        graph: &'env mut BrigadierGraph,
        registry: &'env mut FunctionRegistry,
    ) -> Self {
        match Self::try_new(command, graph, registry) {
            Ok(builder) => builder,
            Err(err) => panic!("Failed to create new Command, {err}"),
        }
    }

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
            Ok(Self {
                command,
                graph,
                registry,
                entrypoint: None,
                previous: None,
                _function: PhantomData,
            })
        }
    }
}

impl<Function> CommandBuilder<'_, Function> {
    /// Build the command using the given function.
    ///
    /// # Panics
    /// Panics if the command could not be built.
    pub fn build<Marker>(self, f: Function)
    where
        Function: IntoFunction<'static, Marker>,
    {
        if let Err(err) = self.try_build(f) {
            panic!("Failed to build Command: {err}");
        }
    }

    /// Build the command using the given function.
    ///
    /// # Errors
    /// Returns an error if the command could not be built or registered.
    pub fn try_build<Marker>(self, f: Function) -> Result<(), BrigadierError>
    where
        Function: IntoFunction<'static, Marker>,
    {
        // Make sure the function has a name.
        let mut dynamic = f.into_function();
        dynamic = match dynamic.name().cloned() {
            Some(name) => dynamic.with_name(format!("brigadier_{name}")),
            None => dynamic.with_name(format!("brigadier_{}", self.command)),
        };

        // Create the command node and register the function.
        let command = BrigadierNode { function: dynamic.name().cloned() };
        self.registry.register(dynamic)?;

        if let (Some((previous, edge)), Some(entrypoint)) = (self.previous, self.entrypoint) {
            // Add the command node to the graph.
            let current = self.graph.graph.add_node(command);
            self.graph.graph.add_edge(previous, current, edge);

            // Set the command entrypoint.
            self.graph.commands.insert(self.command, entrypoint);
        } else {
            // Add the command node to the graph.
            let entrypoint = self.graph.graph.add_node(command);

            // Set the command entrypoint.
            self.graph.commands.insert(self.command, entrypoint);
        }

        Ok(())
    }

    /// Add a string literal to the command.
    #[must_use]
    pub fn literal(mut self, literal: impl Into<SmolStr>) -> Self {
        self.add_edge(BrigadierEdge::Literal(literal.into()));
        self
    }

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

        // If the entrypoint is not set, set it to the new node.
        if self.entrypoint.is_none() {
            self.entrypoint = Some(current);
        }

        // Add an edge connecting the previous node to the new node.
        if let Some((previous, edge)) = self.previous.take() {
            self.graph.graph.add_edge(previous, current, edge);
        }

        // Store the new node and edge for the next call.
        self.previous = Some((current, edge));
    }
}
