//! [`SystemSets`](SystemSet) for loading and saving settings.

use bevy_app::{App, PostUpdate, PreStartup, Startup};
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreStartup, SettingsPreStartupSet)
        .configure_sets(Startup, SettingsStartupSet)
        .configure_sets(PostUpdate, SettingsPostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`PreStartup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct SettingsPreStartupSet;

/// A [`SystemSet`] that runs during the [`Startup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct SettingsStartupSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct SettingsPostUpdateSet;