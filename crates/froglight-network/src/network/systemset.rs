use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, NetworkPreUpdateSet)
        .configure_sets(PostUpdate, NetworkPostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct NetworkPreUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct NetworkPostUpdateSet;
