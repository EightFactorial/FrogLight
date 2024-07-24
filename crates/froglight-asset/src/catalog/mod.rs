use bevy_app::{App, Plugin};
use bevy_render::{mesh::Mesh, texture::Image};

#[allow(clippy::module_inception)]
mod catalog;
pub use catalog::AssetCatalog;

mod key;
pub use key::AssetKey;

mod register;

mod systemset;

/// A [`Plugin`] that adds the [`AssetCatalog`] resource.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetCatalogPlugin;

impl Plugin for AssetCatalogPlugin {
    fn build(&self, app: &mut App) {
        systemset::build(app);
        catalog::build(app);

        // Register the types of AssetKeys
        register::add_systems::<Image>(app);
        register::add_systems::<Mesh>(app);
    }
}
