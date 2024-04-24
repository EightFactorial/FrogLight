use bevy_app::{App, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{ClientPreUpdateSet, PhysicsPostUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreUpdate, UtilityPreUpdateSet.after(ClientPreUpdateSet))
        .configure_sets(Update, UtilityUpdateSet)
        .configure_sets(PostUpdate, UtilityPostUpdateSet.after(PhysicsPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityPreUpdateSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityPostUpdateSet;
