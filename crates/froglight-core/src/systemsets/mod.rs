//! [`SystemSets`](bevy_ecs::prelude::SystemSet) used by all `FrogLight` crates.

use bevy_app::App;

mod interface;
pub use interface::{
    InterfacePostUpdateSet, InterfacePreUpdateSet, InterfaceStartupSet, InterfaceUpdateSet,
};

mod assets;
pub use assets::{AssetPostUpdateSet, AssetPreStartupSet, AssetStartupSet, AssetUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    interface::build(app);
    assets::build(app);
}
