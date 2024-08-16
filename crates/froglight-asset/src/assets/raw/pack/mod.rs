//! [`ResourcePack`], [`ResourcePackMeta`], and other related types.
//!
//! Also contains the [`ResourcePackZipLoader`], [`ResourcePackFolderLoader`],
//! and [`ResourcePackMetaLoader`] asset loaders.

use bevy_app::App;

pub mod loader;
// pub use loader::{ResourcePackFolderLoader, ResourcePackMetaLoader,
// ResourcePackZipLoader};

#[allow(clippy::module_inception)]
mod pack;
pub use pack::ResourcePack;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    pack::build(app);
    loader::build(app);
}
