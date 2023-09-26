use belly::prelude::BellyPlugin;
use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::{render_resource::AddressMode, texture::ImageSampler},
    window::ExitCondition,
};
use bevy_rapier3d::prelude::RapierPhysicsPlugin;
use rand::seq::IteratorRandom;

#[cfg(feature = "splash")]
pub(crate) mod splash;

#[cfg(feature = "debug")]
mod debug;

mod mc_rs;
use mc_rs::MCRSPlugins;

use crate::systems::settings::Settings;

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
    app.add_plugins(RapierPhysicsPlugin::<()>::default());

    #[cfg(feature = "rapier_debug")]
    {
        use bevy_rapier3d::render::RapierDebugRenderPlugin;
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    // Add custom plugins
    MCRSPlugins.build().finish(app);
}

/// Configure the default Bevy [DefaultPlugins].
fn default_plugins(settings: &Settings) -> PluginGroupBuilder {
    let mut plugins = DefaultPlugins.build();

    // Enable asset hot-reloading
    #[cfg(feature = "debug")]
    {
        use bevy::asset::ChangeWatcher;
        use std::time::Duration;

        plugins = plugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            ..default()
        });
    }

    // Set the ImagePlugin sampling to nearest
    {
        let mut default_sampler = ImageSampler::nearest_descriptor();
        default_sampler.address_mode_u = AddressMode::Repeat;
        default_sampler.address_mode_v = AddressMode::Repeat;
        default_sampler.address_mode_w = AddressMode::Repeat;

        plugins = plugins.set(ImagePlugin { default_sampler })
    }

    // Set the WindowPlugin window settings
    {
        let title = match cfg!(debug_assertions) {
            true => {
                format!(
                    "MC-RS v{} - nightly {} - {}",
                    env!("CARGO_PKG_VERSION"),
                    env!("VERGEN_GIT_DESCRIBE"),
                    get_title()
                )
            }
            false => format!("MC-RS v{} - {}", env!("CARGO_PKG_VERSION"), get_title()),
        };

        let window = Window {
            present_mode: settings.window.present_mode,
            mode: settings.window.window_mode,
            resolution: settings.window.resolution.clone(),
            title,
            ..default()
        };

        plugins = plugins.set(WindowPlugin {
            primary_window: Some(window),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        });
    }

    plugins
}

const WINDOW_TITLES: &str = include_str!("../../assets/texts/titles.txt");

/// Get a random window title.
///
/// All occurrences of `\n` will be replaced with a newline.
fn get_title() -> String {
    let mut rng = rand::thread_rng();

    WINDOW_TITLES
        .lines()
        .choose(&mut rng)
        .unwrap_or(WINDOW_TITLES.lines().next().unwrap())
        .replace("\\n", "\n")
}
