use bevy::prelude::Color;
use serde::{Deserialize, Serialize};

/// Backgrounds for the main menu
///
/// TODO: Add backgrounds
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MainMenuBackground {
    /// 3D cubemaps
    CubeMap(CubeMapBackground),
    /// 2D images
    Image(ImageBackground),
    /// Solid colors
    Solid(ColorBackground),
}

impl MainMenuBackground {
    // TODO: Add functions for creating the background
}

impl Default for MainMenuBackground {
    fn default() -> Self { Self::CubeMap(CubeMapBackground::default()) }
}

/// Cube map backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CubeMapBackground {
    #[default]
    Plains,
    Village,
    Desert,
    DesertVillage,
    Ocean,
    WarmOcean,
    Mountains,
    Cave,
    Cavern,
}

/// Image backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageBackground {
    #[default]
    Plains,
}

/// Solid color bbackgrounds for the main menu
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
