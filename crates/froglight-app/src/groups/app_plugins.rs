use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins,
};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] for all plugins that are used in the main application.
///
/// This includes bevy's [`DefaultPlugins`], so no need to add them yourself!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        DefaultPlugins::build(DefaultPlugins).add(CorePlugin).add(NetworkPlugins)
    }
}
