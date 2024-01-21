//! `SystemSets` used for loading screens

use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.configure_sets(Update, LoadingScreenUpdateSet.ambiguous_with_all());
}

/// A [`SystemSet`] that runs loading screen
/// systems during the [`Update`] schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingScreenUpdateSet;
