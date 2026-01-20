//! TODO

use bevy_app::{App, Plugin};

/// A [`Plugin`] that adds networking features and systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, _app: &mut App) {}
}
