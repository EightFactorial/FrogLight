use bevy::{app::AppExit, prelude::*};
use serde::{Deserialize, Serialize};

pub mod camera;
use camera::CameraSettings;

pub mod resourcepack;
use resourcepack::ResourcePackSettings;

pub mod window;
use window::WindowSettings;

use self::window::GuiScale;

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
        app.add_systems(Update, Self::save_config.run_if(on_event::<AppExit>()));

        app.add_systems(
            Update,
            (
                CameraSettings::update_camera,
                WindowSettings::update_window,
                GuiScale::update_scale,
            )
                .run_if(resource_exists_and_changed::<Settings>()),
        );
    }
}

impl Settings {
    pub(crate) fn insert_resources(&self, app: &mut App) {
        app.insert_resource(self.window.resolution.gui_scale);
    }
}
