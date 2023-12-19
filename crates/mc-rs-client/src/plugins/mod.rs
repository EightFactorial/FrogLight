//! Plugins for the client

use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
    window::ExitCondition,
};
use rand::seq::IteratorRandom;

pub mod configs;
use configs::{settings::Settings, traits::ConfigFile};

pub mod resourcepack;

/// Set the default image sampler to nearest and the address mode to repeat.
pub(super) fn image_plugin(plugins: PluginGroupBuilder) -> PluginGroupBuilder {
    let mut default_sampler = ImageSamplerDescriptor::nearest();

    default_sampler.address_mode_u = ImageAddressMode::Repeat;
    default_sampler.address_mode_v = ImageAddressMode::Repeat;
    default_sampler.address_mode_w = ImageAddressMode::Repeat;

    plugins.set(ImagePlugin { default_sampler })
}

/// Set the window title, resolution, vsync, etc.
pub(super) fn window_plugin(plugins: PluginGroupBuilder) -> PluginGroupBuilder {
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

    // I know this loads the Settings twice, but I want it to log if it fails to load later
    plugins.set(WindowPlugin {
        primary_window: Some(Settings::load().window.into_window(title)),
        exit_condition: ExitCondition::OnPrimaryClosed,
        close_when_requested: true,
    })
}

static WINDOW_TITLES: &str = include_str!("../../../../assets/texts/titles.txt");
/// Get a random window title.
fn get_title() -> String {
    let mut rng = rand::thread_rng();

    WINDOW_TITLES
        .lines()
        .choose(&mut rng)
        .unwrap_or(WINDOW_TITLES.lines().next().unwrap())
        .to_string()
}
