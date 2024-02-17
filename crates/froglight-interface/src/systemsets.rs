use bevy::prelude::*;

/// Configure [`SystemSet`]s
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, InterfaceStartupSet);
    app.configure_sets(Update, InterfaceUpdateSet);
}

/// A [`SystemSet`] for systems that run during [`Startup`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InterfaceStartupSet;

/// A [`SystemSet`] for systems that run during [`Update`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InterfaceUpdateSet;
