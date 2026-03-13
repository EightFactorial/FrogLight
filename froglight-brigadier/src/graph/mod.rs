//! TODO

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use petgraph::prelude::*;

mod traits;
pub use traits::AddGameCommand;

/// A graph of containing a tree of command nodes.
#[derive(Debug, Default, Clone, Reflect, Resource)]
#[reflect(opaque, Debug, Default, Clone, Resource)]
pub struct CommandGraph(StableDiGraph<(), ()>);

impl CommandGraph {
    /// Create a new, empty [`CommandGraph`].
    #[must_use]
    pub fn new() -> Self { Self(StableDiGraph::new()) }

    /// Get a reference to the underlying graph.
    #[inline]
    #[must_use]
    pub fn as_graph(graph: &Self) -> &StableDiGraph<(), ()> { &graph.0 }

    /// Get a mutable reference to the underlying graph.
    #[inline]
    #[must_use]
    pub fn as_graph_mut(graph: &mut Self) -> &mut StableDiGraph<(), ()> { &mut graph.0 }
}
