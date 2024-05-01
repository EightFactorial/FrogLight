//! Client [`SystemSet`]s.

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::{common_conditions::any_with_component, IntoSystemSetConfigs, SystemSet};
use froglight_network::connection::{
    systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet},
    ConnectionTask,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        PreUpdate,
        EventPreUpdateSet.after(NetworkPreUpdateSet).run_if(any_with_component::<ConnectionTask>),
    )
    .configure_sets(
        PostUpdate,
        EventPostUpdateSet
            .before(NetworkPostUpdateSet)
            .run_if(any_with_component::<ConnectionTask>),
    );
}

/// A [`SystemSet`] that runs after the [`NetworkPreUpdateSet`] during the
/// [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct EventPreUpdateSet;

/// A [`SystemSet`] that runs before the [`NetworkPostUpdateSet`] during the
/// [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct EventPostUpdateSet;
