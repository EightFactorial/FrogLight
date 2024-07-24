use bevy_app::{App, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};
use bevy_state::{
    app::AppExtStates,
    state::{States, SubStates},
};

use super::{AssetLoadState, AssetLoadSystemSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Create the `AssetStateSystemSet` and initialize the `AssetState` state.
    app.configure_sets(
        Update,
        AssetStateSystemSet.ambiguous_with(AssetLoadSystemSet).after(AssetLoadSystemSet),
    );
    app.init_state::<AssetState>().enable_state_scoped_entities::<AssetState>();

    // Configure `AssetState::Unloaded`
    app.configure_sets(
        Update,
        AssetState::Unloaded.ambiguous_with(AssetState::Loaded).in_set(AssetStateSystemSet),
    );

    // Configure `AssetState::Loaded`
    app.configure_sets(
        Update,
        AssetState::Loaded
            .ambiguous_with(AssetState::Unloaded)
            .after(AssetState::Unloaded)
            .in_set(AssetStateSystemSet),
    );
}

/// A [`SystemSet`] that contains all [`AssetState`] systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetStateSystemSet;

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
