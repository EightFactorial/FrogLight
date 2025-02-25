//! TODO

use std::marker::PhantomData;

use bevy_ecs::entity::Entity;

mod build;
pub use build::FunctionBuilder;

mod world;
pub use world::WorldRef;

use crate::{graph::BrigadierGraph, prelude::ArgumentParser};

/// A builder for adding commands to a [`BrigadierGraph`].
pub struct CommandBuilder<'env, Function> {
    graph: &'env mut BrigadierGraph,
    _function: PhantomData<Function>,
}

#[allow(dead_code)]
impl<'env> CommandBuilder<'env, fn(Entity, WorldRef)> {
    /// Create a new [`CommandBuilder`] for a [`BrigadierGraph`].
    #[inline]
    #[must_use]
    pub(crate) fn new(graph: &'env mut BrigadierGraph) -> Self {
        Self { graph, _function: PhantomData }
    }
}

impl<'env, Function> CommandBuilder<'env, Function> {
    /// Add an argument to the function.
    #[inline]
    #[must_use]
    pub fn argument<Parser: ArgumentParser, NewFunction>(self) -> CommandBuilder<'env, NewFunction>
    where
        Self: FunctionBuilder<'env, Parser, Function, NewFunction>,
    {
        <Self as FunctionBuilder<'env, Parser, Function, NewFunction>>::argument(self)
    }

    /// Build the command using the given function.
    #[expect(unused_variables)]
    pub fn build(self, f: Function) {}

    /// Convert the function to a different type.
    ///
    /// # Note
    /// [`FunctionBuilder`] calls this internally,
    /// so this applies to all building functions.
    #[inline]
    #[must_use]
    fn convert<Other>(self) -> CommandBuilder<'env, Other> {
        CommandBuilder { graph: self.graph, _function: PhantomData }
    }
}

#[test]
fn test() {
    let mut graph = BrigadierGraph::default();
    let builder = CommandBuilder::new(&mut graph);

    let builder = builder.argument::<u32, _>();
    let builder = builder.argument::<f64, _>();
    let builder = builder.argument::<String, _>();

    builder.build(|_entity, _num, _float, _string, _world| {});
}
