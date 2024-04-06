use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, NetworkingPreUpdateSet)
        .configure_sets(PostUpdate, NetworkingPostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct NetworkingPreUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct NetworkingPostUpdateSet;
