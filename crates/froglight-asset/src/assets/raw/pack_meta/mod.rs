//! [`ResourcePackMeta`], and other related types.
//!
//! Also contains the [`ResourcePackMetaLoader`] and
//! [`ResourcePackMetaZipLoader`] asset loaders.

use bevy_app::App;

mod loader;
pub use loader::{ResourcePackMetaLoader, ResourcePackMetaZipLoader};

pub mod asset;
pub use asset::ResourcePackMeta;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    asset::build(app);
    loader::build(app);
}
