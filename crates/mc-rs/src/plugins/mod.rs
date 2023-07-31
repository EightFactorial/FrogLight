use belly::prelude::BellyPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier3d::{prelude::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

mod mc_rs;
use mc_rs::MCRSPlugins;

mod settings;
use settings::Settings;

/// Add plugins to the [App].
///
/// Plugins added changes depending on the enabled features.
pub(super) fn add_plugins(app: &mut App) {
    let settings = Settings::load();

    // Add default plugins
    default_plugins(&settings).finish(app);
    app.insert_resource(settings);

    // Add Belly plugin
    app.add_plugins(BellyPlugin);

    // Add Rapier physics plugins
    app.add_plugins((
        RapierPhysicsPlugin::<()>::default(),
        RapierDebugRenderPlugin::default(),
    ));

    // Add custom plugins
    MCRSPlugins.build().finish(app);
}

/// Configure the default Bevy [DefaultPlugins].
fn default_plugins(settings: &Settings) -> PluginGroupBuilder {
    let mut plugins = DefaultPlugins.build();

    // Set the ImagePlugin sampling to nearest
    {
        plugins = plugins.set(ImagePlugin::default_nearest())
    }

    // Set the WindowPlugin window settings
    {
        let title = match cfg!(debug_assertions) {
            true => {
                format!(
                    "MC-RS v{} - nightly {}",
                    env!("CARGO_PKG_VERSION"),
                    env!("VERGEN_GIT_DESCRIBE")
                )
            }
            // TODO: Add random title from list on startup
            false => format!("MC-RS v{}", env!("CARGO_PKG_VERSION")),
        };

        let window = Window {
            title,
            resolution: settings.window.resolution.clone(),
            ..default()
        };

        plugins = plugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        });
    }

    plugins
}
