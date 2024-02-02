use bevy::prelude::*;
use big_space::{FloatingOrigin, FloatingOriginPlugin, FloatingOriginSettings};

/// The [`Plugin`] for the [`froglight-world`](crate) crate.
///
/// Adds [`Systems`](bevy::ecs::system::System) and
/// [`Components`](bevy::ecs::component::Component) for managing
/// [`Chunks`](crate::world::Chunk).
///
/// Relies on [`big_space`]'s [`FloatingOrigin`] system.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FloatingOriginPlugin::<i16>::new(512.0, 8.0));
    }

    fn finish(&self, app: &mut App) {
        let grid = app.world.resource::<FloatingOriginSettings>();

        // Create an empty grid and position
        let (grid, vec) = grid.translation_to_grid::<i16>([0.0; 3]);

        // Add a blank origin entity to prevent crashing
        let pos = SpatialBundle::from_transform(Transform::from_translation(vec));
        app.world.spawn((FloatingOrigin, grid, pos));

        // Add a blank grid entity to prevent crashing
        let pos = SpatialBundle::from_transform(Transform::from_translation(vec));
        app.world.spawn((grid, pos));
    }
}
