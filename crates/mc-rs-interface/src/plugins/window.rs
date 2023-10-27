use bevy::{
    prelude::*,
    window::{ExitCondition, WindowPlugin as BevyWindowPlugin},
};
use rand::seq::IteratorRandom;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
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
            // present_mode: settings.window.present_mode,
            // mode: settings.window.window_mode,
            // resolution: settings.window.resolution.clone(),
            title,
            ..default()
        };

        app.add_plugins(BevyWindowPlugin {
            primary_window: Some(window),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        });
    }
}

static WINDOW_TITLES: &str = include_str!("../../../../assets/texts/titles.txt");

/// Get a random window title.
///
/// All occurrences of `\n` will be replaced with a newline.
fn get_title() -> String {
    WINDOW_TITLES
        .lines()
        .choose(&mut rand::thread_rng())
        .unwrap_or(WINDOW_TITLES.lines().next().unwrap())
        .to_string()
}
