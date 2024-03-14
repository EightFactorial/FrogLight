use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::plugins::prelude::*;

/// All `Froglight` plugins that can run in a headless environment.
///
/// Only use this group if you know what you're doing!
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ReflectPlugin)
            .add(CorePlugin)
            .add(PhysicsPlugin)
            .add(WorldPlugin)
            .add(ResolverPlugin::default())
            .add(NetworkPlugin)
    }
}

#[test]
fn client_build() { ClientPlugins.build(); }
