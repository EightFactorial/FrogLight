use bevy_app::{App, PostUpdate, Update};
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, WorldUpdateSet).configure_sets(PostUpdate, WorldPostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct WorldUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct WorldPostUpdateSet;
