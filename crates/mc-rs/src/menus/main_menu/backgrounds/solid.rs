use belly::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{menus::MenuRoot, systems::settings::Settings};

use super::{BackgroundEnum, MainMenuBackground};

/// A marker component for the main menu background color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundColor;

/// Solid color backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum BackgroundColorEnum {
    #[default]
    Black,
    #[serde(alias = "grey")]
    Gray,
    #[serde(alias = "darkgrey")]
    DarkGray,
}

impl From<BackgroundColorEnum> for Color {
    fn from(value: BackgroundColorEnum) -> Self {
        match value {
            BackgroundColorEnum::Black => Color::BLACK,
            BackgroundColorEnum::Gray => Color::GRAY,
            BackgroundColorEnum::DarkGray => Color::DARK_GRAY,
        }
    }
}

impl BackgroundColorEnum {
    pub(super) fn create(
        root: Res<MenuRoot>,
        settings: Res<Settings>,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        let BackgroundEnum::Color(bg) = &settings.menu.main_menu else {
            return;
        };

        commands.entity(**root).insert(BackgroundColor);
        commands.entity(**root).insert(MainMenuBackground);

        let bg = bg.to_string();
        elements.select(".root").add_child(eml! {
            <div class="main-background" s:background-color={bg}></div>
        });
    }

    pub(super) fn destroy(root: Res<MenuRoot>, mut elements: Elements, mut commands: Commands) {
        commands.entity(**root).remove::<BackgroundColor>();
        commands.entity(**root).remove::<MainMenuBackground>();

        elements.select(".root div.main-background").remove();
    }
}
