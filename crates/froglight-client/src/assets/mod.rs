//! Asset systems and resources.

use bevy::app::App;

mod managers;
pub use managers::*;

mod resourcepacks;
pub use resourcepacks::ResourcePackSettings;

mod systemsets;
pub use systemsets::{AssetLoading, AssetUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    systemsets::build(app);

    managers::build(app);
    resourcepacks::build(app);
}
