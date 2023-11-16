use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow, Window, WindowMode},
};
use serde::{Deserialize, Serialize};

use super::Settings;

mod resolution;
pub use resolution::*;

mod scale;
pub use scale::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowSettings {
    #[serde(default)]
    pub vsync: PresentMode,
    #[serde(default)]
    pub window: WindowMode,
    #[serde(default)]
    pub resolution: WindowResolution,
}

impl WindowSettings {
    pub fn into_window(self, title: String) -> Window {
        Window {
            title,
            present_mode: self.vsync,
            mode: self.window,
            resolution: self.resolution.into(),
            ..Default::default()
        }
    }

    /// Update the window settings.
    pub(super) fn update_window(
        mut query: Query<&mut Window, With<PrimaryWindow>>,
        settings: Res<Settings>,
    ) {
        if let Ok(mut window) = query.get_single_mut() {
            if settings.window.vsync != window.present_mode {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Updating VSync to {:?}", settings.window.vsync);

                window.present_mode = settings.window.vsync;
            }

            if settings.window.window != window.mode {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Updating window mode to {:?}", settings.window.window);

                window.mode = settings.window.window;
            }

            let new_resolution = (&settings.window.resolution).into();
            if new_resolution != window.resolution {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Updating resolution to {:?}", new_resolution);

                window.resolution = new_resolution;
            }
        }
    }
}
