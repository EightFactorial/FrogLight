use bevy::{app::PluginGroupBuilder, prelude::*};

mod mc_rs;
use mc_rs::MCRSPlugins;

/// Add plugins to the [App].
///
/// This includes the Bevy [DefaultPlugins],
/// as well as several custom plugins from [MCRSPlugins].
///
/// Plugins added changes depending on the enabled features.
pub(super) fn add_plugins(app: &mut App) {
    default_plugins().finish(app);
    MCRSPlugins.build().finish(app);
}

/// Configure the default Bevy [DefaultPlugins].
fn default_plugins() -> PluginGroupBuilder {
    let mut plugins = DefaultPlugins.build();

    // Set the ImagePlugin sampling to nearest
    {
        plugins = plugins.set(ImagePlugin::default_nearest())
    }

    // Set the WindowPlugin window title
    {
        plugins = plugins.set(WindowPlugin {
            primary_window: Some(Window {
                // TODO: Add random title from list on startup
                title: format!("MC-RS v{}", env!("CARGO_PKG_VERSION"),),
                ..default()
            }),
            ..default()
        });
    }

    plugins
}
