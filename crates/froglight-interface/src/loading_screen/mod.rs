//! A loading screen displayed while the client is loading.

use bevy::app::{App, Plugin};

/// A [`Plugin`] that creates a loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, _app: &mut App) {}
}
