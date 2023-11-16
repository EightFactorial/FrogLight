use bevy::window::WindowResolution as BevyWindowResolution;
use mc_rs_gui::resources::gui::GuiScale;
use serde::{Deserialize, Serialize};

use super::scale::GuiScaleSettings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowResolution {
    #[serde(default = "WindowResolution::default_width")]
    pub width: u32,
    #[serde(default = "WindowResolution::default_height")]
    pub height: u32,
    #[serde(default)]
    pub gui_scale: GuiScaleSettings,
}

impl Default for WindowResolution {
    fn default() -> Self {
        Self {
            width: WindowResolution::default_width(),
            height: WindowResolution::default_height(),
            gui_scale: GuiScaleSettings::Auto,
        }
    }
}

impl WindowResolution {
    fn default_width() -> u32 { 960 }
    fn default_height() -> u32 { 720 }
}

impl From<WindowResolution> for BevyWindowResolution {
    fn from(value: WindowResolution) -> Self { Self::new(value.width as f32, value.height as f32) }
}

impl From<&WindowResolution> for BevyWindowResolution {
    fn from(value: &WindowResolution) -> Self { Self::new(value.width as f32, value.height as f32) }
}

impl From<&WindowResolution> for GuiScale {
    fn from(value: &WindowResolution) -> Self {
        value.gui_scale.to_guiscale(value.width, value.height)
    }
}
