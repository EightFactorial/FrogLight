use bevy_app::{App, Plugin};

/// The [`Plugin`] for the [`froglight-world`](crate) crate.
///
/// Adds [`Systems`](bevy_ecs::system::System) and
/// [`Components`](bevy_ecs::component::Component) for managing
/// [`Chunks`](crate::world::Chunk).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {}
}
