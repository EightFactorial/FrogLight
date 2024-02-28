//! `SystemSets` used for resource packs

use bevy_app::{App, PostUpdate, PreStartup, Startup, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(PreStartup, AssetPreStartupSet.ambiguous_with_all());
    app.configure_sets(Startup, AssetStartupSet.ambiguous_with_all());
    app.configure_sets(Update, AssetUpdateSet.ambiguous_with_all());
    app.configure_sets(PostUpdate, AssetPostUpdateSet.ambiguous_with_all());
}

/// A [`SystemSet`] for asset systems that should run during [`PreStartup`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetPreStartupSet;

/// A [`SystemSet`] for asset systems that should run during [`Startup`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetStartupSet;

/// A [`SystemSet`] for asset systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetUpdateSet;

/// A [`SystemSet`] for asset systems that should run during [`PostUpdate`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetPostUpdateSet;
