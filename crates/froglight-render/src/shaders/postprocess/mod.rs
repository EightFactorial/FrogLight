//! Shader modules for rendering.

use bevy::{prelude::*, render::render_graph::RenderSubGraph};

mod portal_effect;
pub use portal_effect::PortalEffect;

mod world_blur;
pub use world_blur::WorldBlurEffect;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Build post-processing shaders.
    portal_effect::build(app);
    world_blur::build(app);

    // TODO: Create the post-process subgraph.

    // TODO: Add the post-process subgraph to the render graph.
}

/// A render subgraph that applies post-processing effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, RenderSubGraph)]
pub struct PostProcessSubGraph;

impl PostProcessSubGraph {
    /// The name of the subgraph.
    pub const GRAPH_NAME: &'static str = "froglight_postprocess";
}

impl FromWorld for PostProcessSubGraph {
    fn from_world(_world: &mut World) -> Self { todo!() }
}
