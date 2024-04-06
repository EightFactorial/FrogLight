use bevy_app::{App, PostUpdate, Startup, Update};
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, InterfaceStartupSet)
        .configure_sets(Update, InterfaceUpdateSet)
        .configure_sets(PostUpdate, InterfacePostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`Startup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfaceStartupSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfaceUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfacePostUpdateSet;
