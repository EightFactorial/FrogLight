//! Client [`SystemSet`]s.

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};
use froglight_network::connection::systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, EventPreUpdateSet.after(NetworkPreUpdateSet))
        .configure_sets(PostUpdate, EventPostUpdateSet.before(NetworkPostUpdateSet));
}

/// A [`SystemSet`] that runs after the [`NetworkPreUpdateSet`] during the
/// [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct EventPreUpdateSet;

/// A [`SystemSet`] that runs before the [`NetworkPostUpdateSet`] during the
/// [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct EventPostUpdateSet;
