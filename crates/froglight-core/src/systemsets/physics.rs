use bevy_app::{App, PostUpdate, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::ClientPostUpdateSet;

/// All `Physics` [`SystemSets`](SystemSet) run after `World`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, PhysicsUpdateSet)
        .configure_sets(PostUpdate, PhysicsPostUpdateSet.after(ClientPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct PhysicsUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct PhysicsPostUpdateSet;
