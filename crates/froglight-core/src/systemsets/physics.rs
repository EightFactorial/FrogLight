use bevy_app::{App, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{WorldPostUpdateSet, WorldPreUpdateSet, WorldUpdateSet};

/// All `Physics` [`SystemSets`](SystemSet) run after `World`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, PhysicsPreUpdateSet.after(WorldPreUpdateSet))
        .configure_sets(Update, PhysicsUpdateSet.after(WorldUpdateSet))
        .configure_sets(PostUpdate, PhysicsPostUpdateSet.after(WorldPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct PhysicsPreUpdateSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct PhysicsUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct PhysicsPostUpdateSet;
