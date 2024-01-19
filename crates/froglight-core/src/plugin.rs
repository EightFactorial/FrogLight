use bevy::prelude::*;

/// The [`Plugin`] for the [`froglight-core`](crate) crate.
///
/// Adds [`Events`](Event) and [`Schedules`](Schedule)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        crate::events::setup(app);
        crate::systemsets::setup(app);
        crate::resources::setup(app);
    }
}
