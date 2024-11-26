use bevy_app::{App, Plugin};

/// The `Entity` plugin for Froglight.
///
/// Adds entities and components for querying entities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "reflect")]
        {
            crate::generated::component::register(app);
            crate::generated::entity::register(app);
        }
    }
}
