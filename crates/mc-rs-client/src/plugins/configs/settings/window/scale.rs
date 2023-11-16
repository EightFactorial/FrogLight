use bevy::prelude::*;
use mc_rs_gui::resources::gui::GuiScale;

use super::Settings;

/// Update the GUI scale if the [Settings] have changed.
pub fn update_scale(settings: Res<Settings>, mut scale: ResMut<GuiScale>) {
    if settings.window.resolution.gui_scale != *scale {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!(
            "Updating GUI scale to {:?}",
            settings.window.resolution.gui_scale
        );

        *scale = settings.window.resolution.gui_scale;
    }
}
