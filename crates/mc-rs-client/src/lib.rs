#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier3d::plugin::RapierPhysicsPlugin;
use mc_rs_core::CorePlugin;
use mc_rs_gui::GuiPlugin;
use mc_rs_network::NetworkingPlugin;
use mc_rs_resourcepack::{assets::ResourceAssetPlugin, pack::ResourcePackPlugin};
use mc_rs_world::WorldPlugin;
use plugins::{configs::ConfigPlugin, resourcepack::ResourcePackSourcePlugin};

mod dir;

pub mod core;
pub mod net;
pub mod plugins;
pub mod res;
pub mod world;

/// A [`PluginGroup`] that adds all the [`Plugin`](Plugin)s needed for the client.
///
/// By default this loads Bevy's [`DefaultPlugins`],
/// but this can be turned off by disabling the `default_plugins` feature.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[cfg(feature = "default_plugins")]
        let mut plugins = DefaultPlugins::build(DefaultPlugins);
        #[cfg(not(feature = "default_plugins"))]
        let mut plugins = PluginGroupBuilder::start::<ClientPlugins>();

        // Add required plugins
        plugins = plugins
            .add_before::<AssetPlugin, ResourcePackSourcePlugin>(ResourcePackSourcePlugin)
            .add(RapierPhysicsPlugin::<()>::default())
            .add(ResourcePackPlugin)
            .add(ResourceAssetPlugin)
            .add(CorePlugin)
            .add(GuiPlugin)
            .add(WorldPlugin)
            .add(NetworkingPlugin);

        #[cfg(feature = "default_plugins")]
        {
            // Add the config plugin after the log plugin if it's enabled
            #[cfg(feature = "debug")]
            {
                plugins = plugins.add_after::<bevy::log::LogPlugin, ConfigPlugin>(ConfigPlugin);
            }
            // Disable the log plugin if the debug feature is disabled
            #[cfg(not(feature = "debug"))]
            {
                plugins = plugins.disable::<bevy::log::LogPlugin>();
            }

            // Set the default image sampler to nearest and the address mode to repeat
            plugins = plugins::image_plugin(plugins);
            // Set the window title, resolution, vsync, etc.
            plugins = plugins::window_plugin(plugins);
        }
        // Add the config plugin without any conditions if the log plugin is disabled
        #[cfg(any(not(feature = "default_plugins"), not(feature = "debug")))]
        {
            plugins = plugins.add(ConfigPlugin);
        }

        // Add the rapier debug plugin if the debug_rapier feature is enabled
        #[cfg(feature = "debug_rapier")]
        {
            app.add_plugins(bevy_rapier3d::render::RapierDebugRenderPlugin::default());
        }

        plugins
    }
}
