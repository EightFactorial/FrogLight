use bevy_app::{App, Plugin};

/// The [`Plugin`] for the [`froglight-protocol`](crate) crate.
///
/// Registers all types for reflection.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReflectPlugin;

impl Plugin for ReflectPlugin {
    fn build(&self, _app: &mut App) {}
}
