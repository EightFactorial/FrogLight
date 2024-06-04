//! [`SystemSets`](SystemSet) for running utility systems.

use bevy_app::{App, PreUpdate};
#[cfg(feature = "froglight-network")]
use bevy_ecs::schedule::IntoSystemSetConfigs;
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    #[cfg(not(feature = "froglight-network"))]
    app.configure_sets(PreUpdate, UtilityPreUpdateSet);

    #[cfg(feature = "froglight-network")]
    app.configure_sets(
        PreUpdate,
        UtilityPreUpdateSet.after(froglight_network::network::NetworkPreUpdateSet),
    );
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityPreUpdateSet;
