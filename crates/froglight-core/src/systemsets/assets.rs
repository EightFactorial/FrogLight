use bevy_app::{App, PreStartup, Startup, Update};
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreStartup, AssetPreStartupSet)
        .configure_sets(Startup, AssetStartupSet)
        .configure_sets(Update, AssetUpdateSet);
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
