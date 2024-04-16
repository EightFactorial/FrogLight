use bevy_app::{App, PostUpdate, PreStartup};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{NetworkPostUpdateSet, NetworkPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreStartup, EntityPreUpdateSet.after(NetworkPreUpdateSet))
        .configure_sets(PostUpdate, EntityPostUpdateSet.after(NetworkPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct EntityPreUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct EntityPostUpdateSet;
