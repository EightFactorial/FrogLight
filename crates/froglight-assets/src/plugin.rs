//! Asset plugins for `FrogLight`
use std::sync::Arc;

use bevy_app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ecs::schedule::BoxedCondition;
use parking_lot::Mutex;

#[cfg(feature = "asset_manager")]
pub use crate::asset_manager::plugin::AssetManagerPlugin;
pub use crate::{
    asset_source::plugin::AssetSourcePlugin, resourcepack::plugin::ResourcePackPlugin,
    settings::plugin::SettingsPlugin,
};

/// A [`PluginGroup`] for asset related plugins.
///
/// Can be used to add most asset related plugins to an [`App`].
///
/// ---
///
/// Does not include the [`AssetSourcePlugin`], as it requires being added
/// before bevy's [`AssetPlugin`](bevy_asset::AssetPlugin).
#[derive(Default, Clone)]
pub struct AssetPlugins {
    pub(crate) conditions: Arc<Mutex<Vec<BoxedCondition>>>,
}

impl AssetPlugins {
    /// The default folder name for the asset source.
    pub const DEFAULT_FOLDER: &'static str = "FrogLight";

    /// Create a new [`AssetPlugins`]
    #[must_use]
    pub fn new() -> Self { Self { conditions: Arc::new(Mutex::new(Vec::new())) } }

    /// Add a condition that must be met before `ResourcePack`s are considered
    /// finished processing.
    pub fn add_condition(&self, condition: BoxedCondition) {
        self.conditions.lock().push(condition);
    }
}

impl PluginGroup for AssetPlugins {
    #[allow(unused_mut, clippy::let_and_return)]
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(SettingsPlugin)
            .add(ResourcePackPlugin::from_conditions_arc(self.conditions));

        // Add the `AssetManagerPlugin` if the `asset_manager` feature is enabled
        #[cfg(feature = "asset_manager")]
        {
            group = group.add(AssetManagerPlugin::default());
        }

        group
    }
}

impl Plugin for AssetPlugins {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        super::systemset::build(app);

        // Add `Self` as a plugin group
        <Self as PluginGroup>::build(self.clone()).finish(app);
    }
}
