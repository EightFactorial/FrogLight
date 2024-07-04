//! [`SystemSets`](SystemSet) for running utility systems.

use bevy_app::{App, PreUpdate};
#[cfg(feature = "froglight-network")]
use bevy_ecs::schedule::apply_deferred;
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    #[cfg(not(feature = "froglight-network"))]
    app.configure_sets(PreUpdate, UtilityPreUpdateSet);

    #[cfg(feature = "froglight-network")]
    app.configure_sets(
        PreUpdate,
        UtilityPreUpdateSet.after(froglight_network::network::NetworkPreUpdateSet),
    );

    // Create the `UtilityPreUpdateDeferedSet` that runs `apply_deferred`
    app.configure_sets(PreUpdate, UtilityPreUpdateDeferredSet.in_set(UtilityPreUpdateSet));
    app.add_systems(PreUpdate, apply_deferred.in_set(UtilityPreUpdateDeferredSet));
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityPreUpdateSet;

/// A [`SystemSet`] that runs [`apply_deferred`] inside the
/// [`UtilityPreUpdateSet`].
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityPreUpdateDeferredSet;
