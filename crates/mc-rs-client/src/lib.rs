#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};
use mc_rs_core::CorePlugin;
use mc_rs_gui::GuiPlugin;
use mc_rs_network::NetworkingPlugin;
use plugins::{configs::ConfigPlugin, resourcepack::ResourcePackSourcePlugin};

mod dir;

pub mod core;
pub mod net;
pub mod plugins;
pub mod res;

/// A plugin group that adds all the plugins needed for the client.
///
/// By default this loads Bevy's [DefaultPlugins],
/// but this can be turned off by disabling the `default-plugins` feature.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[cfg(feature = "default_plugins")]
        let mut plugins = DefaultPlugins::build(DefaultPlugins);
        #[cfg(not(feature = "default_plugins"))]
        let mut plugins = PluginGroupBuilder::start::<ClientPlugins>();

        // Disable the log plugin if the default plugins
        // are enabled and the debug feature is disabled
        #[cfg(all(feature = "default_plugins", not(feature = "debug")))]
        {
            plugins = plugins.disable::<bevy::log::LogPlugin>();
        }

        // Set the default image sampler to nearest and the address mode to repeat
        plugins = plugins::image_plugin(plugins);
        // Set the window title, resolution, vsync, etc.
        plugins = plugins::window_plugin(plugins);

        // Add required plugins
        plugins
            .add_before::<AssetPlugin, ResourcePackSourcePlugin>(ResourcePackSourcePlugin)
            .add_after::<LogPlugin, ConfigPlugin>(ConfigPlugin)
            .add(CorePlugin)
            .add(GuiPlugin)
            .add(NetworkingPlugin)
    }
}
