use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::{render_resource::AddressMode, texture::ImageSampler},
    window::ExitCondition,
};
use mc_rs_interface::settings::Settings;
use rand::seq::IteratorRandom;

/// Configure the default Bevy [DefaultPlugins].
pub(crate) fn default_plugins(settings: &Settings) -> PluginGroupBuilder {
    let mut plugins = DefaultPlugins.build();

    // Enable asset hot-reloading
    #[cfg(any(debug_assertions, feature = "debug"))]
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
        let title: String;

        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            title = format!(
                "MC-RS v{} - nightly {} - {}",
                env!("CARGO_PKG_VERSION"),
                env!("VERGEN_GIT_DESCRIBE"),
                get_title()
            );
        }
        #[cfg(not(any(debug_assertions, feature = "debug")))]
        {
            title = format!("MC-RS v{} - {}", env!("CARGO_PKG_VERSION"), get_title());
        }

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

static WINDOW_TITLES: &str = include_str!("../../assets/texts/titles.txt");

/// Get a random window title.
///
/// All occurrences of `\n` will be replaced with a newline.
fn get_title() -> String {
    WINDOW_TITLES
        .lines()
        .choose(&mut rand::thread_rng())
        .unwrap_or(WINDOW_TITLES.lines().next().unwrap())
        .replace("\\n", "\n")
}
