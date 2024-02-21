use bevy::prelude::*;
use froglight_core::systemsets::InterfaceUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, InspectorUpdateSet.in_set(InterfaceUpdateSet));
}

/// A [`SystemSet`] for inspector systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InspectorUpdateSet;
