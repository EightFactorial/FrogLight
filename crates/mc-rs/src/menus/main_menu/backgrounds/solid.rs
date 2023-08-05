use belly::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::Display;

/// Solid color backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum BackgroundColorEnum {
    #[default]
    Black,
    DarkGray,
    Gray,
}

impl From<BackgroundColorEnum> for Color {
    fn from(value: BackgroundColorEnum) -> Self {
        match value {
            BackgroundColorEnum::Black => Color::BLACK,
            BackgroundColorEnum::DarkGray => Color::DARK_GRAY,
            BackgroundColorEnum::Gray => Color::GRAY,
        }
    }
}

impl BackgroundColorEnum {
    pub(super) fn create(&self, mut elements: Elements) {
        let color = self.to_string();

        elements.select(".root").add_child(eml! {
            <div class="main-background" s:background-color={color}></div>
        });
    }
}
