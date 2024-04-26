//! Client [`SystemSet`]s.

use bevy::{
    app::{App, PreUpdate},
    ecs::schedule::{IntoSystemSetConfigs, SystemSet},
};
use froglight_core::systemsets::{ClientPreUpdateSet, NetworkPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        PreUpdate,
        ClientNetworkingSet.after(NetworkPreUpdateSet).in_set(ClientPreUpdateSet),
    );
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClientNetworkingSet;
