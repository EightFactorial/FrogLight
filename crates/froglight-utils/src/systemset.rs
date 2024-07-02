//! [`SystemSets`](SystemSet) for running utility systems.

use bevy_app::{App, PreUpdate};
#[cfg(feature = "froglight-network")]
use bevy_ecs::schedule::apply_deferred;
#[cfg(feature = "froglight-network")]
use bevy_ecs::schedule::IntoSystemConfigs;
#[cfg(feature = "froglight-network")]
use bevy_ecs::schedule::IntoSystemSetConfigs;
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    #[cfg(not(feature = "froglight-network"))]
    app.configure_sets(PreUpdate, UtilityPreUpdateSet);

    #[cfg(feature = "froglight-network")]
    {
        use froglight_network::network::NetworkPreUpdateSet;

        // Create the `UtilityPreUpdateDeferedSet` that runs
        // `apply_deferred` after `NetworkPreUpdateSet`.
        app.configure_sets(PreUpdate, UtilityPreUpdateDeferedSet.after(NetworkPreUpdateSet));
        app.add_systems(PreUpdate, apply_deferred.in_set(UtilityPreUpdateDeferedSet));

        // Create the `UtilityPreUpdateSet` that runs
        // after `UtilityPreUpdateDeferedSet`.
        app.configure_sets(PreUpdate, UtilityPreUpdateSet.after(UtilityPreUpdateDeferedSet));
    }
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityPreUpdateSet;

/// A [`SystemSet`] that runs [`apply_deferred`] during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
#[cfg(feature = "froglight-network")]
struct UtilityPreUpdateDeferedSet;
