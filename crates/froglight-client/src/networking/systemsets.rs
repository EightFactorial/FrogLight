//! Client Networking [`SystemSet`]s.

use bevy::{
    app::{App, PostUpdate, PreUpdate},
    ecs::schedule::{IntoSystemSetConfigs, SystemSet},
};
use froglight_network::connection::systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet};
use froglight_utils::systemsets::UtilityPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        PreUpdate,
        ClientNetworkPreUpdateSet.after(NetworkPreUpdateSet).before(UtilityPreUpdateSet),
    )
    .configure_sets(PostUpdate, ClientNetworkPostUpdateSet.before(NetworkPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClientNetworkPreUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClientNetworkPostUpdateSet;
