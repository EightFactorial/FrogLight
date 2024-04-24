#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};
use bevy_ecs::schedule::BoxedCondition;
use parking_lot::Mutex;

pub mod assets;

mod asset_manager;
pub use asset_manager::AssetManager;

mod events;
pub use events::*;

mod settings;
pub use settings::ResourcePackSettings;

mod states;
pub use states::AssetLoadingState;

/// The `Assets` Froglight plugin.
///
/// Adds asset loading and management to the app.
#[derive(Default)]
pub struct AssetPlugin {
    conditions: Mutex<Vec<BoxedCondition>>,
}

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        states::build(app);
        events::build(app);

        asset_manager::build(app);
        assets::build(app);

        settings::build(app);
    }

    fn finish(&self, app: &mut App) {
        // Add the state change conditions
        states::finish(self, app);
    }
}
