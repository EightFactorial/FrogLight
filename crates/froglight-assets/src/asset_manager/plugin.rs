use bevy::prelude::*;

use super::manager::AssetManager;
use crate::ResourcePackLoader;

/// Adds the `AssetManager` resource to the app.
///
/// All assets loaded for `[ResourcePack]`s will be automatically
/// inserted into the `AssetManager` resource.
#[derive(Debug, Default, Clone)]
pub struct AssetManagerPlugin(Option<AssetManager>);

impl AssetManagerPlugin {
    /// Create a new `AssetManagerPlugin`.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Create a new `AssetManagerPlugin` with the given `AssetManager`.
    #[must_use]
    pub fn from(manager: AssetManager) -> Self { Self(Some(manager)) }
}

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::systemset::build(app);

        // Register and add the `AssetManager` resource
        let manager = self.0.clone().unwrap_or_default();
        app.register_type::<AssetManager>().insert_resource(manager.clone());

        // Add the `ResourcePackLoader` asset loader
        debug!("Initializing ResourcePackLoader");
        app.register_asset_loader(ResourcePackLoader(manager));

        // Build the `AssetTracker` resource
        super::tracker::build(app);
    }
}
