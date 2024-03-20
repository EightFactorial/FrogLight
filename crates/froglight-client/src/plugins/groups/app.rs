use bevy::{app::PluginGroupBuilder, prelude::*, winit::WinitPlugin};

use crate::plugins::prelude::*;

/// All `Froglight` plugins.
///
/// This is a collection of all the plugins that are part of the `Froglight`
/// client, including bevy's [`DefaultPlugins`].
///
/// // TODO: Add example of how to use this plugin group.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroup::build(DefaultPlugins)
            .set(WinitPlugin { run_on_any_thread: true })
            .add_before::<AssetPlugin, AssetSourcePlugin>(AssetSourcePlugin::default())
            .add(ReflectPlugin)
            .add(CorePlugin)
            .add(WorldPlugin)
            .add(PhysicsPlugin)
            .add(AssetPlugins::default())
            .add(NetworkPlugins)
            .add(DebugPlugins)
            .add(InterfacePlugins)
    }
}

#[test]
fn app_build() { AppPlugins.build(); }
