use bevy::{app::AppExit, prelude::*};
use serde::{Deserialize, Serialize};

pub mod camera;
use camera::CameraSettings;

pub mod resourcepack;
use resourcepack::ResourcePackSettings;

pub mod window;
use window::{GuiScaleSettings, WindowSettings};

use super::traits::{ConfigFile, ResourceConfig};

#[derive(Debug, Default, Clone, PartialEq, Resource, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub camera: CameraSettings,
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
        app.add_systems(
            Update,
            (GuiScaleSettings::update_settings, Self::save_config)
                .chain()
                .run_if(on_event::<AppExit>()),
        );

        app.add_systems(
            PreUpdate,
            (
                CameraSettings::update_camera,
                WindowSettings::update_window,
                GuiScaleSettings::update_scale,
            )
                .run_if(resource_exists_and_changed::<Settings>()),
        );
    }
}

impl Settings {
    pub(crate) fn insert_resources(&self, app: &mut App) {
        // Insert GuiScaleSettings
        {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!(
                "Inserting GuiScaleSettings: {:?}",
                self.window.resolution.gui_scale
            );

            app.insert_resource(self.window.resolution.gui_scale);
        }

        // Insert GuiScale
        {
            let scale = self
                .window
                .resolution
                .gui_scale
                .to_guiscale(self.window.resolution.width, self.window.resolution.height);

            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Inserting GuiScale: {scale:?}");

            app.insert_resource(scale);
        }
    }
}
