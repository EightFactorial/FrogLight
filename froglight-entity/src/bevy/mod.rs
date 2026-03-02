//! TODO

use bevy_app::{App, Plugin};

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, _app: &mut App) {}
}
