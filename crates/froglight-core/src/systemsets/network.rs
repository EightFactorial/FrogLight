use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::PhysicsPostUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, NetworkPreUpdateSet)
        .configure_sets(PostUpdate, NetworkPostUpdateSet.after(PhysicsPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct NetworkPreUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct NetworkPostUpdateSet;
