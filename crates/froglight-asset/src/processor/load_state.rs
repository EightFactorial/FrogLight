use bevy_app::{App, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};
use bevy_state::{app::AppExtStates, state::States};

use super::AssetStateSystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Create the `AssetStateSystemSet` and initialize the `AssetState` state.
    app.configure_sets(Update, AssetLoadSystemSet.ambiguous_with(AssetStateSystemSet));
    app.init_state::<AssetLoadState>();

    // Configure `AssetLoadState::Waiting`
    app.configure_sets(
        Update,
        AssetLoadState::Waiting
            .ambiguous_with(AssetLoadState::Loading)
            .ambiguous_with(AssetLoadState::Processing)
            .ambiguous_with(AssetLoadState::Spawning)
            .ambiguous_with(AssetLoadState::Finished)
            .in_set(AssetLoadSystemSet),
    );

    // Configure `AssetLoadState::Loading`
    app.configure_sets(
        Update,
        AssetLoadState::Loading
            .ambiguous_with(AssetLoadState::Waiting)
            .ambiguous_with(AssetLoadState::Processing)
            .ambiguous_with(AssetLoadState::Spawning)
            .ambiguous_with(AssetLoadState::Finished)
            .after(AssetLoadState::Waiting)
            .in_set(AssetLoadSystemSet),
    );

    // Configure `AssetLoadState::Processing`
    app.configure_sets(
        Update,
        AssetLoadState::Processing
            .ambiguous_with(AssetLoadState::Waiting)
            .ambiguous_with(AssetLoadState::Loading)
            .ambiguous_with(AssetLoadState::Spawning)
            .ambiguous_with(AssetLoadState::Finished)
            .after(AssetLoadState::Loading)
            .in_set(AssetLoadSystemSet),
    );

    // Configure `AssetLoadState::Spawning`
    app.configure_sets(
        Update,
        AssetLoadState::Spawning
            .ambiguous_with(AssetLoadState::Waiting)
            .ambiguous_with(AssetLoadState::Loading)
            .ambiguous_with(AssetLoadState::Processing)
            .ambiguous_with(AssetLoadState::Finished)
            .after(AssetLoadState::Processing)
            .in_set(AssetLoadSystemSet),
    );

    // Configure `AssetLoadState::Finished`
    app.configure_sets(
        Update,
        AssetLoadState::Finished
            .ambiguous_with(AssetLoadState::Waiting)
            .ambiguous_with(AssetLoadState::Loading)
            .ambiguous_with(AssetLoadState::Processing)
            .ambiguous_with(AssetLoadState::Spawning)
            .after(AssetLoadState::Spawning)
            .in_set(AssetLoadSystemSet),
    );
}

/// A [`SystemSet`] that contains all [`AssetLoadState`] systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetLoadSystemSet;

/// The state of the asset loading process.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet, States)]
pub enum AssetLoadState {
    /// Waiting for assets to be loaded.
    #[default]
    Waiting,
    /// Loading assets.
    Loading,
    /// Processing loaded assets.
    Processing,
    /// Spawning entities from loaded assets.
    Spawning,
    /// Finished loading assets.
    Finished,
}
