use bevy::prelude::*;
use froglight_core::systemsets::InterfaceUpdateSet;

use crate::plugins::debug::systemset::DebugUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, InspectorUpdateSet.after(DebugUpdateSet).in_set(InterfaceUpdateSet));
}

/// A [`SystemSet`] for inspector systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InspectorUpdateSet;
