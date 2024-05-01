//! [`SystemSets`](SystemSet) for running utility systems.

use bevy_app::{App, PreUpdate};
use bevy_ecs::schedule::SystemSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.configure_sets(PreUpdate, UtilityPreUpdateSet); }

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UtilityPreUpdateSet;
