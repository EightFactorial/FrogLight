use bevy::window::{PresentMode, WindowMode, WindowResolution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowSettings {
    #[serde(default)]
    pub vsync: PresentMode,
    #[serde(default)]
    pub window: WindowMode,
    #[serde(default)]
    pub resolution: WindowResolution,
}
