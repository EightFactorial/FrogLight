use bevy::prelude::*;

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
        crate::map::build(app);
        crate::world::build(app);
    }
}
