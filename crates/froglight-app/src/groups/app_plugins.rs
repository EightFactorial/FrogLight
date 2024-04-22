use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetPlugin as BevyAssetPlugin,
    DefaultPlugins as BevyDefaultPlugins,
};

use crate::{prelude::plugins::*, FrogLightPlugins};

/// A [`PluginGroup`] for all plugins that are used in the [`FrogLight`](crate)
/// application.
///
/// ---
///
/// This includes bevy's [`DefaultPlugins`](BevyDefaultPlugins), so no need to
/// add them yourself!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        BevyDefaultPlugins::build(BevyDefaultPlugins)
            .add_before::<BevyAssetPlugin, _>(SettingsPlugin::default())
            .add(FrogLightPlugins)
    }
}
