use bevy::prelude::*;

/// The [`Plugin`] for the [`froglight-core`](crate) crate.
///
/// Adds [`Events`](Event) and [`Schedules`](Schedule)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Register events
        crate::events::build(app);
        // Setup system sets
        crate::systemsets::build(app);
        // Initialize resources
        crate::resources::build(app);
    }
}
