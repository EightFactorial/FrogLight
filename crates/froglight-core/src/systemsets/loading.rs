//! `SystemSets` used for loading screens

use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<LoadingScreenUpdateSet>()
        .configure_sets(Update, LoadingScreenUpdateSet.ambiguous_with_all());
}

/// A [`SystemSet`] that runs loading screen
/// systems during the [`Update`] schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, Reflect)]
pub struct LoadingScreenUpdateSet;
