//! The progress bar of the loading screen
use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn setup(_app: &mut App) {}

/// The progress bar of the loading screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct ProgressBar;

impl ProgressBar {
    pub(super) fn build(_app: &mut App, _parent: Entity) {}
}
