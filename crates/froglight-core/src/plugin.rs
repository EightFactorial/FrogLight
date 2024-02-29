use bevy_app::{App, Plugin};

/// The [`Plugin`] for the [`froglight-core`](crate) crate.
///
/// Adds [`Events`](Event) and [`Schedules`](Schedule)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Register components
        crate::components::build(app);
        // Initialize resources
        crate::resources::build(app);
        // Register events
        crate::events::build(app);
        // Setup system sets
        crate::systemsets::build(app);
    }
}
