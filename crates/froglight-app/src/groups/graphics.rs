use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] for all plugins that are part of the graphical `FrogLight`
/// application.
///
/// This includes:
/// - [`AssetPlugin`]
/// - [`RenderPlugin`]
/// - [`InterfacePlugin`]
/// - [`ClientPlugin`]
///
/// ## Note
///
/// This **does not** include the [`SettingsPlugin`], which must be added
/// before bevy's [`AssetPlugin`](bevy::asset::AssetPlugin).
/// ```rust,no_run
/// use bevy::{asset::AssetPlugin, prelude::*, DefaultPlugins};
/// use froglight_app::prelude::plugins::SettingsPlugin;
///
/// let mut app = App::new();
///
/// // Add the `SettingsPlugin` before the `AssetPlugin`
/// let default_plugins = DefaultPlugins.build();
/// app.add_plugins(default_plugins.add_before::<AssetPlugin, _>(SettingsPlugin::default()));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphicalPlugins;

impl PluginGroup for GraphicalPlugins {
    fn build(self) -> PluginGroupBuilder { Self::add(PluginGroupBuilder::start::<Self>()) }
}

impl GraphicalPlugins {
    /// Adds all the [`Plugins`](bevy::prelude::Plugin) that are part of the
    /// [`GraphicalPlugins`] [`PluginGroup`].
    pub(crate) fn add(builder: PluginGroupBuilder) -> PluginGroupBuilder {
        builder.add(AssetPlugin).add(RenderPlugin).add(InterfacePlugin).add(ClientPlugin)
    }
}
