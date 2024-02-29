use bevy::prelude::{App, Plugin};

/// The [`Plugin`] for the [`froglight-render`](crate) crate.
///
/// Adds world and entity rendering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Add the shaders.
        super::shaders::build(app);
    }
}
