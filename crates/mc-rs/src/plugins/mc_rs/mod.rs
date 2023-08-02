use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct MCRSPlugins;

impl PluginGroup for MCRSPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut plugins = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "splash")]
        {
            use super::splash::SplashPlugin;
            plugins = plugins.add(SplashPlugin);
        }

        #[cfg(debug_assertions)]
        {
            use super::debug::DebugPlugin;
            plugins = plugins.add(DebugPlugin);
        }

        plugins
    }
}
