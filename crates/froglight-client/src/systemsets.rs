//! Client [`SystemSet`]s.

use bevy::{
    app::{App, PostUpdate, PreUpdate, Update},
    ecs::schedule::SystemSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, ClientPreUpdateSet)
        .configure_sets(Update, ClientUpdateSet)
        .configure_sets(PostUpdate, ClientPostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClientPreUpdateSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClientUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClientPostUpdateSet;
