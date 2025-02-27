//! TODO

use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_reflect::func::{FunctionRegistry, IntoFunction};
use smol_str::SmolStr;

pub mod build;
pub use build::CommandBuilder;
use build::{Arg, BuilderState, Command, Fork};

mod traits;
use traits::FunctionBuilder;

mod world;
pub use world::{Empty, Full, WorldRef};

use crate::{
    graph::{BrigadierEdge, BrigadierGraph},
    prelude::ArgumentParser,
};

impl<'env> CommandBuilder<'env, Arg, fn(Entity, WorldRef<Full>)> {
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
}

// You can add arguments during the `Arg` state.
// You can add a command during the `Arg` state.
impl<'env, Function> CommandBuilder<'env, Arg, Function> {
    /// Add a string literal to the command.
    #[must_use]
    pub fn literal(mut self, literal: impl Into<SmolStr>) -> CommandBuilder<'env, Arg, Function> {
        self.add_edge(BrigadierEdge::literal(literal));
        self.convert()
    }

    /// Add an argument to the function.
    #[must_use]
    #[expect(private_bounds)]
    pub fn arg<Parser: ArgumentParser, NewFunction>(
        mut self,
    ) -> CommandBuilder<'env, Arg, NewFunction>
    where
        Self: FunctionBuilder<'env, Parser, Arg, Function, NewFunction>,
    {
        self.add_edge(BrigadierEdge::argument::<Parser>());
        self.argument().convert()
    }

    /// Build the command using the given function.
    ///
    /// # Panics
    /// Panics if the command could not be built.
    pub fn command<Marker>(self, f: Function) -> CommandBuilder<'env, Command, Function>
    where
        Function: IntoFunction<'static, Marker>,
    {
        match self.try_command(f) {
            Ok(builder) => builder,
            Err(err) => {
                panic!("Failed to build Command: {err}");
            }
        }
    }

    /// Fork the command builder to split the command into multiple paths.
    #[inline]
    pub fn fork(self, mut f: impl FnMut(CommandBuilder<'env, Fork, Function>)) {
        f(self.convert());
    }
}

// You can add arguments during the `Command` state.
// You cannot add a command from the `Command` state.
impl<'env, Function> CommandBuilder<'env, Command, Function> {
    /// Add a string literal to the command.
    #[must_use]
    pub fn literal(mut self, literal: impl Into<SmolStr>) -> CommandBuilder<'env, Arg, Function> {
        self.add_edge(BrigadierEdge::literal(literal));
        self.convert()
    }

    /// Add an argument to the function.
    #[must_use]
    #[expect(private_bounds)]
    pub fn arg<Parser: ArgumentParser, NewFunction>(
        mut self,
    ) -> CommandBuilder<'env, Arg, NewFunction>
    where
        Self: FunctionBuilder<'env, Parser, Command, Function, NewFunction>,
    {
        self.add_edge(BrigadierEdge::argument::<Parser>());
        self.argument().convert()
    }
}

// You cannot add arguments or commands during the `Fork` state.
impl<Function> CommandBuilder<'_, Fork, Function> {
    /// Start a new forked command.
    #[must_use]
    pub fn start(&mut self) -> CommandBuilder<'_, Command, Function> {
        CommandBuilder {
            command: self.command.clone(),
            entrypoint: self.entrypoint,
            previous: self.previous,
            graph: self.graph,
            registry: self.registry,
            _phantom: PhantomData,
        }
    }
}
