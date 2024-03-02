use bevy_app::{App, Plugin};

/// The [`Plugin`] for the [`froglight-world`](crate) crate.
///
/// Adds [`Systems`](bevy::ecs::system::System) and
/// [`Components`](bevy::ecs::component::Component) for managing
/// [`Chunks`](crate::world::Chunk).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        crate::blocks::build(app);
        crate::biomes::build(app);
        crate::maps::build(app);
        crate::world::build(app);
    }
}
