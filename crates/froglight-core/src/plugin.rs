use bevy::prelude::*;
use big_space::{FloatingOrigin, FloatingOriginPlugin, FloatingOriginSettings};

/// The [`Plugin`] for the [`froglight-core`](crate) crate.
///
/// Adds [`Events`](Event) and [`Schedules`](Schedule)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Max size (2^bits * edge_length) = 2^16 * 512 = 33,554,432
        app.add_plugins(FloatingOriginPlugin::<i16>::new(512.0, 8.0));

        // Register events
        crate::events::build(app);
        // Setup system sets
        crate::systemsets::build(app);
        // Initialize resources
        crate::resources::build(app);
    }

    fn finish(&self, app: &mut App) {
        // TODO: Remove this once the bug(?) is fixed
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
