//! `SystemSets` for the loading screen.
use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) { app.configure_sets(Startup, LoadingScreenStartupSet); }

/// A [`SystemSet`] that runs once at [`Startup`] to setup the loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct LoadingScreenStartupSet;
