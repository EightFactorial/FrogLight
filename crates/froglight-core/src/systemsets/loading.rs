//! `SystemSets` used for loading screens

use bevy::prelude::*;

use crate::resources::loading::LoadingScreenEnable;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.configure_sets(
        Update,
        LoadingScreenUpdateSet
            .run_if(resource_exists_and_equals(LoadingScreenEnable(true)))
            .run_if(not(resource_added::<LoadingScreenEnable>())),
    );
}

/// A [`SystemSet`] that runs loading screen systems during the [`Update`]
/// schedule.
///
/// Only runs if the [`LoadingScreenEnable`] [`Resource`] is set to `true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingScreenUpdateSet;
