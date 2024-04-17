//! [`SystemSets`](SystemSet) for scheduling the registry systems.

use bevy_app::{PostStartup, PostUpdate, PreUpdate};
use bevy_ecs::schedule::{common_conditions::on_event, IntoSystemSetConfigs, SystemSet};

use super::NetworkPreUpdateSet;
use crate::events::RegistryUpdateEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    // Add the `RegistryPostStartupSet`
    app.configure_sets(PostStartup, RegistryPostStartupSet);

    // Add the `RegistryPreUpdateSet` after the `NetworkPreUpdateSet`
    app.configure_sets(PreUpdate, RegistryPreUpdateSet.after(NetworkPreUpdateSet));

    // Add the `RegistryPostUpdateSet` only run if
    // a `RegistryUpdateEvent` is  received
    app.configure_sets(PostUpdate, RegistryPostUpdateSet.run_if(on_event::<RegistryUpdateEvent>()));
}

/// A [`SystemSet`] that runs during the [`PostStartup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct RegistryPostStartupSet;

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct RegistryPreUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct RegistryPostUpdateSet;
