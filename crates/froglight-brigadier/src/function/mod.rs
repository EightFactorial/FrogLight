//! TODO

use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_reflect::func::{FunctionRegistry, IntoFunction};
use smol_str::SmolStr;

mod build;
pub use build::FunctionBuilder;

#[cfg(test)]
mod test;

use crate::graph::{BrigadierEdge, BrigadierError, BrigadierGraph, BrigadierNode};

/// A builder for adding commands to a [`BrigadierGraph`].
pub struct CommandBuilder<'env, Function> {
    command: SmolStr,
    nodes: Vec<(BrigadierEdge, BrigadierNode)>,
    graph: &'env mut BrigadierGraph,
    registry: &'env mut FunctionRegistry,
    _function: PhantomData<Function>,
}

#[allow(dead_code)]
impl<'env> CommandBuilder<'env, fn(Entity, Entity)> {
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
            Ok(Self { command, nodes: Vec::new(), graph, registry, _function: PhantomData })
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
    /// Returns an error if the command could not be built.
    #[expect(clippy::missing_panics_doc)]
    pub fn try_build<Marker>(self, f: Function) -> Result<(), BrigadierError>
    where
        Function: IntoFunction<'static, Marker>,
    {
        // Make sure the function has a name.
        let mut dynamic = f.into_function();
        if dynamic.name().is_none() {
            dynamic = dynamic.with_name(format!("brigadier_{}", self.command));
        }
        let command = BrigadierNode { function: dynamic.name().cloned() };

        if self.nodes.is_empty() {
            // Add the command node.
            let entrypoint = self.graph.graph.add_node(command);
            // Set the command entrypoint.
            self.graph.commands.insert(self.command, entrypoint);
        } else {
            // Add all the argument parser nodes to the graph.
            let mut nodes = self.nodes.into_iter();

            // Add the first node.
            let (mut edge, node) = nodes.next().unwrap();
            let first_id = self.graph.graph.add_node(node);

            // Add the rest of the nodes.
            let mut id = first_id;
            for (next_edge, next_node) in nodes {
                let next_id = self.graph.graph.add_node(next_node);
                self.graph.graph.add_edge(id, next_id, edge);

                edge = next_edge;
                id = next_id;
            }

            // Add the command node.
            let command_id = self.graph.graph.add_node(command);
            self.graph.graph.add_edge(id, command_id, edge);

            // Set the command entrypoint.
            self.graph.commands.insert(self.command, first_id);
        }

        // Register the function.
        self.registry.register(dynamic).unwrap();

        Ok(())
    }

    /// Add a string literal to the command.
    #[must_use]
    pub fn literal(mut self, literal: impl Into<SmolStr>) -> Self {
        self.nodes.push((BrigadierEdge::Literal(literal.into()), BrigadierNode { function: None }));
        self
    }
}
