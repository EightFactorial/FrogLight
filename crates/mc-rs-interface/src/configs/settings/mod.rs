use bevy::{prelude::*, window::PrimaryWindow};
use serde::{Deserialize, Serialize};

use crate::traits::config::{ConfigFile, ResourceConfig};

pub mod window;
use window::WindowSettings;

pub mod resourcepack;
use resourcepack::ResourcePackSettings;

#[derive(Debug, Default, Clone, PartialEq, Resource, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub window: WindowSettings,
    #[serde(default)]
    pub resourcepacks: ResourcePackSettings,
}

impl ConfigFile for Settings {
    const FILE_PATH: &'static str = "settings.toml";
}
impl ResourceConfig for Settings {
    fn add_systems(app: &mut App) {
        app.add_systems(Update, Self::update_window.run_if(Self::exit_event));

        app.add_systems(
            Update,
            (Self::save_config, Self::update_window).run_if(resource_exists_and_changed::<Self>()),
        );
    }
}

impl Settings {
    /// Update the window settings.
    fn update_window(mut query: Query<&mut Window, With<PrimaryWindow>>, settings: Res<Self>) {
        if let Ok(mut window) = query.get_single_mut() {
            if settings.window.vsync != window.present_mode {
                #[cfg(any(debug_assertions, feature = "debug"))]
                {
                    debug!("Updating VSync to {:?}", settings.window.vsync);
                }

                window.present_mode = settings.window.vsync;
            }

            if settings.window.window != window.mode {
                #[cfg(any(debug_assertions, feature = "debug"))]
                {
                    debug!("Updating window mode to {:?}", settings.window.window);
                }

                window.mode = settings.window.window;
            }

            if settings.window.resolution != window.resolution {
                #[cfg(any(debug_assertions, feature = "debug"))]
                {
                    debug!("Updating resolution to {:?}", settings.window.resolution);
                }

                window.resolution = settings.window.resolution.clone();
            }
        }
    }
}
