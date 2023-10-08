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

        #[cfg(feature = "debug")]
        {
            use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
            plugins = plugins.add(FrameTimeDiagnosticsPlugin);

            use super::debug::DebugPlugin;
            plugins = plugins.add(DebugPlugin);
        }

        plugins
    }
}
