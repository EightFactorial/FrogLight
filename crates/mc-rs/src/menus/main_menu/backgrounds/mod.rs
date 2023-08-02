use bevy::prelude::Color;
use serde::{Deserialize, Serialize};

/// Backgrounds for the main menu
///
/// TODO: Add backgrounds
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MainMenuBackground {
    // 3D backgrounds
    CubeMap(CubeMapBackgrounds),

    // 2D images

    // Solid colors
    Solid(Color),
}

impl MainMenuBackground {
    // TODO: Add functions for creating the background
}

impl Default for MainMenuBackground {
    fn default() -> Self { Self::CubeMap(CubeMapBackgrounds::default()) }
}

/// Cube map backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CubeMapBackgrounds {
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
