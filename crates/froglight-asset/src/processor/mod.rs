use bevy_app::{App, Plugin};

mod loading;
mod process;
mod spawning;

mod sources;
pub use sources::ResourcePackList;

mod state;
pub use state::{AssetLoadState, AssetLoadSystemSet, AssetState, AssetStateSystemSet};

mod trigger;
pub use trigger::{ResourceLoadTrigger, ResourceResetTrigger};

/// A [`Plugin`] that adds asset processing systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetProcessorPlugin;

impl Plugin for AssetProcessorPlugin {
    fn build(&self, app: &mut App) {
        state::build_asset_state(app);
        state::build_load_state(app);

        sources::build(app);
        trigger::build(app);

        loading::build(app);
        process::build(app);
        spawning::build(app);
    }
}
