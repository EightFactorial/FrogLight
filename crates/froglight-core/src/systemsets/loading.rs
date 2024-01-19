//! `SystemSets` used for loading screens

use bevy::prelude::*;

use crate::resources::loading::LoadingScreenEnable;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.configure_sets(Update, LoadingScreenSet.run_if(LoadingScreenSet::condition));
}

/// A [`SystemSet`] that runs the loading screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingScreenSet;

impl LoadingScreenSet {
    /// Returns `true` if the loading screen is enabled.
    fn condition(res: Res<LoadingScreenEnable>) -> bool { **res }
}
