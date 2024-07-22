use bevy_app::App;
use bevy_ecs::schedule::SystemSet;
use bevy_state::{
    app::AppExtStates,
    state::{States, SubStates},
};

use super::AssetLoadState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_state::<AssetState>().enable_state_scoped_entities::<AssetState>();

    // TODO: Configure SystemSets
}

/// The state of all assets.
///
/// Very useful for spawning systems waiting for assets to be loaded.
///
/// # Example:
/// ```rust,ignore
/// app.add_systems(OnEnter(AssetState::Loaded), ...);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet, States)]
pub enum AssetState {
    /// Assets are not loaded.
    #[default]
    Unloaded,
    /// Assets are loaded.
    Loaded,
}

impl SubStates for AssetState {
    type SourceStates = AssetLoadState;
    fn should_exist(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            AssetLoadState::Spawning | AssetLoadState::Finished => Some(Self::Loaded),
            _ => Some(Self::Unloaded),
        }
    }
}
