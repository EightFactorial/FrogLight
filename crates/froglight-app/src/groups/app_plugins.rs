use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins as BevyDefaultPlugins,
};

use crate::FrogLightPlugins;

/// A [`PluginGroup`] for all plugins that are used in the main application.
///
/// ---
///
/// This includes bevy's [`DefaultPlugins`](BevyDefaultPlugins), so no need to
/// add them yourself!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        BevyDefaultPlugins::build(BevyDefaultPlugins).add(FrogLightPlugins)
    }
}
