//! [`ResourcePack`] and other related types.
//!
//! Also contains the [`ResourcePackLoader`] and [`ResourcePackZipLoader`]
//! asset loaders.

use bevy_app::App;

pub mod loader;
pub use loader::{ResourcePackLoader, ResourcePackZipLoader};

#[allow(clippy::module_inception)]
mod asset;
pub use asset::ResourcePack;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    asset::build(app);
    loader::build(app);
}
