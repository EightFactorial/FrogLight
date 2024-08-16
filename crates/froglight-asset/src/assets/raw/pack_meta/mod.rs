//! [`ResourcePackMeta`], and other related types.

use bevy_app::App;

mod loader;
pub use loader::{ResourcePackMetaLoader, ResourcePackMetaZipLoader};

pub mod meta;
pub use meta::ResourcePackMeta;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    meta::build(app);
    loader::build(app);
}
