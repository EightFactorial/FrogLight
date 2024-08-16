//! A catalog of loaded assets.

use bevy_app::{App, Plugin};

mod register;

#[allow(clippy::module_inception)]
mod catalog;
mod catalog_iter;
mod catalog_ref;

pub use catalog::AssetCatalog;
pub use catalog_iter::{CatalogIter, CatalogIterMut};
pub use catalog_ref::{TypedCatalogMut, TypedCatalogRef};

mod key;
pub use key::AssetKey;

/// A [`Plugin`] that adds the [`AssetCatalog`].
///
/// Allows for easy access to assets by name.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CatalogPlugin;

impl Plugin for CatalogPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AssetCatalog>().init_resource::<AssetCatalog>();
    }
}
