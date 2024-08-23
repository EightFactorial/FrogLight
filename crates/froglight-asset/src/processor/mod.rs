//! Systems and resources for processing assets.

use bevy_app::{App, Plugin};

mod sources;
pub use sources::ResourcePackList;

pub mod state;

mod systemset;
pub use systemset::{AssetProcess, AssetProcessSet, AssetState, AssetStateSet};

mod trigger;
pub use trigger::{ResourceLoadTrigger, ResourceResetTrigger};

/// A [`Plugin`] that processes raw asset definitions.
///
/// Also inserts assets into the [`AssetCatalog`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]

pub struct AssetProcessorPlugin;

impl Plugin for AssetProcessorPlugin {
    fn build(&self, app: &mut App) {
        systemset::build(app);

        sources::build(app);
        trigger::build(app);

        state::build(app);
    }
}
