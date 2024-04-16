use bevy_app::{App, PreStartup, Startup, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{SettingsPreStartupSet, SettingsStartupSet, SettingsUpdateSet};

/// All `Asset` [`SystemSets`](SystemSet) run after `Settings`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreStartup, AssetPreStartupSet.after(SettingsPreStartupSet))
        .configure_sets(Startup, AssetStartupSet.after(SettingsStartupSet))
        .configure_sets(Update, AssetUpdateSet.after(SettingsUpdateSet));
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
