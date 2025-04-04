//! TODO

use bevy_app::{App, Plugin};

/// A [`Plugin`] that adds various
/// [`SystemSet`](bevy_ecs::schedule::SystemSet)s to an [`App`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemSetPlugin;

impl Plugin for SystemSetPlugin {
    fn build(&self, _app: &mut App) {}
}
