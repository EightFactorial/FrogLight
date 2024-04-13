//! [`SystemSets`](SystemSet) for scheduling the registry systems.

use bevy_app::{PostStartup, PostUpdate};
use bevy_ecs::schedule::{common_conditions::on_event, IntoSystemSetConfigs, SystemSet};

use crate::RegistryOverrideEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    // Add the `RegistryPostStartupSet`
    app.configure_sets(PostStartup, RegistryPostStartupSet);

    // Add the `RegistryPostUpdateSet`
    app.configure_sets(
        PostUpdate,
        RegistryPostUpdateSet.run_if(on_event::<RegistryOverrideEvent>()),
    );
}

/// A [`SystemSet`] that runs during the [`PostStartup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub(crate) struct RegistryPostStartupSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub(crate) struct RegistryPostUpdateSet;
