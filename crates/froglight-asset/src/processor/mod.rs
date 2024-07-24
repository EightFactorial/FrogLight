use bevy_app::{App, Plugin};

mod loading;

mod sources;
pub use sources::ResourcePackList;

mod states;
pub use states::{AssetLoadState, AssetLoadSystemSet, AssetState, AssetStateSystemSet};

mod trigger;
pub use trigger::{ResourceLoadTrigger, ResourceResetTrigger};

/// A [`Plugin`] that adds asset processing systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetProcessorPlugin;

impl Plugin for AssetProcessorPlugin {
    fn build(&self, app: &mut App) {
        states::build_asset_state(app);
        states::build_load_state(app);

        loading::build(app);
        sources::build(app);
        trigger::build(app);
    }
}
