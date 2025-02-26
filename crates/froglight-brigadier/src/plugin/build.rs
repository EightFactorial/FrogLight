use bevy_app::App;
use bevy_ecs::{entity::Entity, reflect::AppFunctionRegistry, world::World};
use smol_str::SmolStr;

use crate::{function::WorldRef, graph::AppBrigadierGraph, prelude::CommandBuilder};

/// A trait for adding commands to the
/// [`BrigadierGraph`](crate::graph::BrigadierGraph).
pub trait BrigadierBuilder {
    /// Add a command to the [`BrigadierGraph`](crate::graph::BrigadierGraph).
    fn add_command(
        &mut self,
        command: impl Into<SmolStr>,
        f: impl FnMut(CommandBuilder<'_, fn(Entity, WorldRef)>),
    ) -> &mut Self;
}

impl BrigadierBuilder for App {
    #[inline]
    fn add_command(
        &mut self,
        command: impl Into<SmolStr>,
        f: impl FnMut(CommandBuilder<'_, fn(Entity, WorldRef)>),
    ) -> &mut Self {
        self.world_mut().add_command(command, f);
        self
    }
}

impl BrigadierBuilder for World {
    fn add_command(
        &mut self,
        command: impl Into<SmolStr>,
        mut f: impl FnMut(CommandBuilder<'_, fn(Entity, WorldRef)>),
    ) -> &mut Self {
        let graph = self.resource::<AppBrigadierGraph>().clone();
        let mut graph = graph.write();

        let registry = self.resource::<AppFunctionRegistry>().clone();
        let mut registry = registry.write();

        f(CommandBuilder::new(command, &mut graph, &mut registry));

        self
    }
}
