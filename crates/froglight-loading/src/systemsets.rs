//! `SystemSets` for the loading screen.
use bevy::prelude::*;
use froglight_core::resources::loading::LoadingScreenEnable;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.configure_sets(Startup, LoadingScreenStartupSet);

    // Configure sets
    app.configure_sets(
        Update,
        LoadingScreenFadeOutUpdateSet
            .run_if(resource_exists_and_equals(LoadingScreenEnable(false)))
            .run_if(not(resource_added::<LoadingScreenEnable>())),
    );
}

/// A [`SystemSet`] that runs once at [`Startup`] to setup the loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct LoadingScreenStartupSet;

/// A [`SystemSet`] that runs loading screen systems when
/// [`LoadingScreenEnable`] resource is set to `false`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct LoadingScreenFadeOutUpdateSet;
