use bevy_app::{App, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::NetworkPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, ClientPreUpdateSet.after(NetworkPreUpdateSet))
        .configure_sets(Update, ClientUpdateSet)
        .configure_sets(PostUpdate, ClientPostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct ClientPreUpdateSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct ClientUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct ClientPostUpdateSet;
