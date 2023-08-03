use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Solid color backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorBackground {
    #[default]
    Black,
    Gray,
}

impl From<ColorBackground> for Color {
    fn from(value: ColorBackground) -> Self {
        match value {
            ColorBackground::Black => Color::BLACK,
            ColorBackground::Gray => Color::GRAY,
        }
    }
}

impl ColorBackground {
    pub(super) fn create(&self, _parent: Entity, mut _commands: Commands) {
        // TODO: Add backgrounds
    }
}
