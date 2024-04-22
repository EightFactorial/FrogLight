use bevy_app::{App, PostUpdate, PreStartup, Startup, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{SettingsPostUpdateSet, SettingsPreStartupSet, SettingsStartupSet};

/// All `Asset` [`SystemSets`](SystemSet) run after `Settings`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreStartup, AssetPreStartupSet.after(SettingsPreStartupSet))
        .configure_sets(Startup, AssetStartupSet.after(SettingsStartupSet))
        .configure_sets(Update, AssetUpdateSet)
        .configure_sets(PostUpdate, AssetPostUpdateSet.after(SettingsPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`PreStartup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct AssetPreStartupSet;

/// A [`SystemSet`] that runs during the [`Startup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct AssetStartupSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct AssetUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct AssetPostUpdateSet;
