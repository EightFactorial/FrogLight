use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{NetworkPostUpdateSet, NetworkPreUpdateSet, PhysicsPostUpdateSet, PhysicsPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        PreUpdate,
        EntityPreUpdateSet.after(NetworkPreUpdateSet).after(PhysicsPreUpdateSet),
    )
    .configure_sets(
        PostUpdate,
        EntityPostUpdateSet.after(NetworkPostUpdateSet).after(PhysicsPostUpdateSet),
    );
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct EntityPreUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct EntityPostUpdateSet;
