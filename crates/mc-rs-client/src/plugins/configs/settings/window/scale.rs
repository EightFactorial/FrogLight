use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::plugins::configs::settings::Settings;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GuiScale(Option<u32>);

impl GuiScale {
    pub const AUTO: GuiScale = GuiScale(None);

    pub(crate) fn update_scale(settings: Res<Settings>, mut scale: ResMut<GuiScale>) {
        if settings.window.resolution.gui_scale != *scale {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!(
                "Updating GUI scale to {:?}",
                settings.window.resolution.gui_scale
            );

            *scale = settings.window.resolution.gui_scale;
        }
    }

    /// Create a new `GuiScale` from the given width and height.
    pub fn new(width: u32, height: u32) -> GuiScale {
        let val = std::cmp::max(1, std::cmp::min(width / 320, height / 240));
        GuiScale(Some(val))
    }
}
