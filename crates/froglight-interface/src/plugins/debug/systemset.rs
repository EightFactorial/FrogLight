use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, DebugStartupSet);
    app.configure_sets(Update, DebugUpdateSet);
}

/// A [`SystemSet`] for debug systems that should run during [`Startup`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct DebugStartupSet;

/// A [`SystemSet`] for debug systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct DebugUpdateSet;
