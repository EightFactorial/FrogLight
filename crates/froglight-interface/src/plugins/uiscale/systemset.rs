use bevy::prelude::*;
use froglight_core::systemsets::{InterfaceStartupSet, InterfaceUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, UiScaleStartupSet.in_set(InterfaceStartupSet));
    app.configure_sets(Update, UiScaleUpdateSet.in_set(InterfaceUpdateSet));
}

/// A [`SystemSet`] for [`UiScale`] systems that should run during [`Startup`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct UiScaleStartupSet;

/// A [`SystemSet`] for [`UiScale`] systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct UiScaleUpdateSet;
