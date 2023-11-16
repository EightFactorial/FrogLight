use bevy::window::WindowResolution as BevyWindowResolution;
use mc_rs_gui::resources::gui::GuiScale;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowResolution {
    pub width: u32,
    pub height: u32,
    pub gui_scale: GuiScale,
}

impl Default for WindowResolution {
    fn default() -> Self {
        Self {
            width: 960,
            height: 720,
            gui_scale: GuiScale::AUTO,
        }
    }
}

impl From<WindowResolution> for BevyWindowResolution {
    fn from(value: WindowResolution) -> Self { Self::new(value.width as f32, value.height as f32) }
}

impl From<&WindowResolution> for BevyWindowResolution {
    fn from(value: &WindowResolution) -> Self { Self::new(value.width as f32, value.height as f32) }
}
