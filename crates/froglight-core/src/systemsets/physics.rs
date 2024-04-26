use bevy_app::{App, PostUpdate};
use bevy_ecs::schedule::SystemSet;

/// All `Physics` [`SystemSets`](SystemSet) run after `World`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.configure_sets(PostUpdate, PhysicsPostUpdateSet); }

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct PhysicsPostUpdateSet;
