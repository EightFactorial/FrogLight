use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] for *almost* all [`FrogLight`](crate) plugins that are
/// used in the main application.
///
/// ### Note
/// This does not include [`SettingsPlugin`], which must be added
/// before bevy's [`AssetPlugin`](bevy::asset::AssetPlugin).
///
/// If Rust's type inference is not working, you can use
/// [`FrogLightPlugins::as_plugin`] to get a [`Plugin`] or
/// [`FrogLightPlugins::as_plugingroup`] to get a [`PluginGroup`].
///
///
/// # Example
/// ```rust
/// use bevy::app::App;
/// use froglight_app::FrogLightPlugins;
///
/// // As a [`Plugin`] (recommended)
/// let mut app = App::new();
/// app.add_plugins(FrogLightPlugins::as_plugin());
///
/// // Or, as a [`PluginGroup`]
/// let mut app = App::new();
/// app.add_plugins(FrogLightPlugins::as_plugingroup());
/// ```
/// ---
///
/// This **does not include** bevy's [`DefaultPlugins`](bevy::DefaultPlugins),
/// so you need to add them yourself!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrogLightPlugins;

impl FrogLightPlugins {
    /// Get the [`FrogLightPlugins`] [`PluginGroup`] into a [`Plugin`].
    ///
    /// Useful for when you specifically need a [`Plugin`].
    #[must_use]
    pub fn as_plugin() -> impl Plugin { Self }

    /// Get the [`FrogLightPlugins`] [`PluginGroup`].
    ///
    /// Useful for when you specifically need a [`PluginGroup`].
    #[must_use]
    pub fn as_plugingroup() -> impl PluginGroup { Self }
}

impl Plugin for FrogLightPlugins {
    fn build(&self, app: &mut App) { <Self as PluginGroup>::build(Self).finish(app); }
}

impl PluginGroup for FrogLightPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetPlugin)
            .add(CorePlugin)
            .add(EntityPlugin)
            .add(RegistryPlugin)
            .add(UtilityPlugin)
            .add(NetworkPlugins)
            .add(RenderPlugin)
            .add(InterfacePlugin)
            .add(ClientPlugin)
    }
}
