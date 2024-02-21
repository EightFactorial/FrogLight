//! `SystemSets` used for resource packs

use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, InterfaceStartupSet.ambiguous_with_all());
    app.configure_sets(PreUpdate, InterfacePreUpdateSet.ambiguous_with_all());
    app.configure_sets(Update, InterfaceUpdateSet.ambiguous_with_all());
    app.configure_sets(PostUpdate, InterfacePostUpdateSet.ambiguous_with_all());
}

/// A [`SystemSet`] for interface systems that should run during [`Startup`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct InterfaceStartupSet;

/// A [`SystemSet`] for interface systems that should run during [`PreUpdate`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct InterfacePreUpdateSet;

/// A [`SystemSet`] for interface systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct InterfaceUpdateSet;

/// A [`SystemSet`] for interface systems that should run during [`PostUpdate`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct InterfacePostUpdateSet;
