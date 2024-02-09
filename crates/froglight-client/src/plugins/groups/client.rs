use bevy::app::{PluginGroup, PluginGroupBuilder};
use froglight_core::CorePlugin;
use froglight_debug::DebugPlugin;
use froglight_physics::PhysicsPlugin;
use froglight_world::WorldPlugin;

/// A [`PluginGroup`] that includes most [`FrogLight`](crate) plugins.
///
/// This group does not include [`bevy's`](bevy)
/// [`DefaultPlugins`](bevy::DefaultPlugins), use with caution.
///
/// For most use cases, [`AppPlugins`](super::app::AppPlugins) is recommended.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    #[allow(clippy::default_constructed_unit_structs)]
    fn build(self) -> PluginGroupBuilder {
        self.build_group(PluginGroupBuilder::start::<Self>()).add(DebugPlugin::default())
    }
}

impl ClientPlugins {
    #[allow(clippy::unused_self)]
    pub(super) fn build_group(self, group: PluginGroupBuilder) -> PluginGroupBuilder {
        group.add(CorePlugin).add(WorldPlugin).add(PhysicsPlugin)
    }
}

#[test]
fn test_build() { ClientPlugins::build(ClientPlugins); }
