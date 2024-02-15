use bevy::{asset::ReflectHandle, prelude::*};

use crate::{manager::ResourcePackManager, ResourcePack, ResourcePackLoader, ResourcePackTracker};

/// The [`Plugin`] for the [`froglight-resourcepack`](crate) crate.
///
/// Adds support for loading resource packs and asset management.
#[derive(Debug, Default, Clone)]
pub struct ResourcePackPlugin(ResourcePackManager);

impl ResourcePackPlugin {
    /// Creates a new [`ResourcePackPlugin`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Creates a new [`ResourcePackPlugin`] with the given
    /// [`ResourcePackManager`].
    #[must_use]
    pub fn new_from(manager: ResourcePackManager) -> Self { Self(manager) }
}

impl From<ResourcePackManager> for ResourcePackPlugin {
    fn from(manager: ResourcePackManager) -> Self { Self::new_from(manager) }
}

impl Plugin for ResourcePackPlugin {
    fn build(&self, app: &mut App) {
        // Insert the manager into the app's resources
        app.insert_resource(self.0.clone());

        // Register the ResourcePack type and initialize it as an asset
        app.init_asset::<ResourcePack>()
            .register_type::<ResourcePack>()
            .register_type_data::<Handle<ResourcePack>, ReflectHandle>();

        // Register the ResourcePackLoader
        app.register_asset_loader(ResourcePackLoader);

        // Initialize resources
        app.init_resource::<ResourcePackManager>().register_type::<ResourcePackManager>();
        app.init_resource::<ResourcePackTracker>();

        // Configure the ResourcePackTracker
        crate::tracker::build(app);
    }
}
