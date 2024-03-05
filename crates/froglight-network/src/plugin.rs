use bevy_app::{App, Plugin};

/// The [`Plugin`] for the [`froglight-network`](crate) crate.
///
/// Adds networking and multiplayer support.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) { crate::connection::build(app); }
}
