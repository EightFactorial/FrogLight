use bevy_app::{App, Plugin};

mod asset_state;
pub use asset_state::{AssetState, AssetStateSystemSet};

mod load_state;
pub use load_state::{AssetLoadState, AssetLoadSystemSet};

/// A [`Plugin`] that adds asset processing systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetProcessorPlugin;

impl Plugin for AssetProcessorPlugin {
    fn build(&self, app: &mut App) {
        asset_state::build(app);
        load_state::build(app);
    }
}
