use bevy_app::App;
use bevy_ecs::schedule::SystemSet;
use bevy_state::{app::AppExtStates, state::States};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_state::<AssetLoadState>();

    // TODO: Configure SystemSets
}

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
