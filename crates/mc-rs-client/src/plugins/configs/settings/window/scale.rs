use bevy::{prelude::*, window::PrimaryWindow};
use mc_rs_gui::resources::scale::GuiScale;
use serde::{Deserialize, Serialize};

use super::Settings;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Serialize, Deserialize)]
pub enum GuiScaleSettings {
    #[default]
    Auto,
    Fixed(u32),
}

impl GuiScaleSettings {
    /// Update the [`GuiScale`] when the [`Settings`] change.
    pub(crate) fn update_scale(
        settings: Res<Settings>,
        query: Query<&Window, With<PrimaryWindow>>,

        mut scale: ResMut<GuiScale>,
    ) {
        let Ok(window) = query.get_single() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Settings changed but window not found, not updating gui scale");
            return;
        };

        let conv_scale = settings
            .window
            .resolution
            .gui_scale
            .to_guiscale(window.width() as u32, window.height() as u32);

        if conv_scale != *scale {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Settings set GUI scale to {conv_scale:?}",);

            *scale = conv_scale;
        }
    }

    /// Update the [GuiScale] of [Settings] when exiting the game.
    pub(crate) fn update_settings(scale: Res<GuiScale>, mut settings: ResMut<Settings>) {
        let scale = (*scale).into();

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Updating Settings with GuiScale {scale:?}");

        settings.window.resolution.gui_scale = scale;
    }

    /// Convert the [GuiScaleSettings] to a [GuiScale],
    /// using the given width and height in case of [GuiScale::Auto].
    pub fn to_guiscale(&self, width: u32, height: u32) -> GuiScale {
        match self {
            Self::Auto => GuiScale::new_auto(width, height),
            Self::Fixed(val) => GuiScale::new_fixed(*val, width, height),
        }
    }
}

impl From<GuiScale> for GuiScaleSettings {
    fn from(value: GuiScale) -> Self {
        match value {
            GuiScale::Auto(_) => Self::Auto,
            GuiScale::Fixed { max, .. } => Self::Fixed(max),
        }
    }
}
