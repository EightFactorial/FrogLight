//! `SystemSets` for the loading screen.
use bevy::prelude::*;
use froglight_core::{resources::LoadingScreenEnable, systemsets::LoadingScreenUpdateSet};

use crate::layout::{fade_animation::FadeTimer, LoadingScreenRoot};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    // Configure resources
    app.init_resource::<LoadingScreenEnableSystems>();

    // Configure startup sets
    app.configure_sets(Startup, LoadingScreenStartupSet.ambiguous_with_all());

    // Configure toggle set
    app.configure_sets(
        Update,
        LoadingScreenToggleSet
            .in_set(LoadingScreenUpdateSet)
            .run_if(resource_exists_and_equals(LoadingScreenEnableSystems(true))),
    );

    // Configure fade in/out update sets
    app.configure_sets(
        Update,
        (LoadingScreenFadeInSet, LoadingScreenFadeOutSet).chain().in_set(LoadingScreenToggleSet),
    );

    // Configure systems that enable/disable toggle set
    app.add_systems(
        Update,
        LoadingScreenEnableSystems::enable_fade_systems
            .run_if(resource_exists_and_equals(LoadingScreenEnable(true)))
            .run_if(resource_exists_and_changed::<LoadingScreenEnable>())
            .before(LoadingScreenToggleSet)
            .in_set(LoadingScreenUpdateSet),
    );
    app.add_systems(
        Update,
        LoadingScreenEnableSystems::disable_fade_systems
            .run_if(resource_exists_and_equals(LoadingScreenEnable(false)))
            .run_if(not(LoadingScreenRoot::is_visible))
            .run_if(resource_removed::<FadeTimer>())
            .after(LoadingScreenFadeOutSet)
            .in_set(LoadingScreenToggleSet),
    );
}

/// A [`SystemSet`] that runs once at [`Startup`] to setup the loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct LoadingScreenStartupSet;

/// A [`Resource`] that tracks whether loading screen systems should be run.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub(crate) struct LoadingScreenEnableSystems(pub(crate) bool);

impl LoadingScreenEnableSystems {
    pub(crate) fn enable_fade_systems(mut state: ResMut<Self>) {
        debug!("Enabling fade systems...");
        state.0 = true;
    }

    pub(crate) fn disable_fade_systems(mut state: ResMut<Self>) {
        debug!("Disabling fade systems...");
        state.0 = false;
    }
}

/// A [`SystemSet`] that runs loading screen
/// systems when the loading screen is enabled.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingScreenToggleSet;

/// A [`SystemSet`] that runs loading screen
/// systems when fading in.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingScreenFadeInSet;

/// A [`SystemSet`] that runs loading screen
/// systems when fading out.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingScreenFadeOutSet;
