use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowMode, WindowResolution},
};
use serde::{Deserialize, Serialize};

use super::Settings;

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Update,
        WindowSettings::update_window.run_if(resource_exists_and_changed::<Settings>()),
    );
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowSettings {
    pub vsync_mode: PresentMode,
    pub window_mode: WindowMode,
    pub resolution: WindowResolution,
}

impl WindowSettings {
    fn update_window(mut query: Query<&mut Window, With<PrimaryWindow>>, settings: Res<Settings>) {
        let Ok(mut window) = query.get_single_mut() else {
            error!("Settings changed, but no primary window found to update!");
            return;
        };

        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            debug!("Updating primary window settings");
        }

        window.mode = settings.window.window_mode;
        window.resolution = settings.window.resolution.clone();
        window.present_mode = settings.window.vsync_mode;
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            vsync_mode: PresentMode::AutoVsync,
            window_mode: WindowMode::Windowed,
            resolution: WindowResolution::default(),
        }
    }
}
