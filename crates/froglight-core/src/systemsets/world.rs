use bevy_app::{App, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{NetworkPostUpdateSet, NetworkPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, WorldPreUpdateSet.after(NetworkPreUpdateSet))
        .configure_sets(Update, WorldUpdateSet)
        .configure_sets(PostUpdate, WorldPostUpdateSet.after(NetworkPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct WorldPreUpdateSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct WorldUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct WorldPostUpdateSet;
