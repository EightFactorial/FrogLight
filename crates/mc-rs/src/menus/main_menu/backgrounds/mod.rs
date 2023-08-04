use belly::prelude::Elements;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{menus::MenuRoot, systems::settings::Settings};

use self::{cubemap::CubeMapBackground, image::ImageBackground, solid::ColorBackground};

pub mod cubemap;
pub mod image;
pub mod solid;

pub(super) fn setup_backgrounds(_app: &mut App) {
    // TODO
}

/// A marker component for the main menu background
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuBackground;

impl MainMenuBackground {
    /// Create the background
    #[allow(dead_code)]
    fn create(
        root: Res<MenuRoot>,
        settings: Res<Settings>,
        elements: Elements,
        mut commands: Commands,
    ) {
        let entity = **root;

        commands.entity(entity).insert(MainMenuBackground);
        match &settings.menu.main_menu {
            BackgroundEnum::CubeMap(bg) => bg.create(entity, elements, commands),
            BackgroundEnum::Image(bg) => bg.create(entity, elements, commands),
            BackgroundEnum::Solid(bg) => bg.create(entity, elements, commands),
        }
    }

    /// Destroy the background
    #[allow(dead_code)]
    fn destroy(
        query: Query<Entity, With<MainMenuBackground>>,
        mut _elements: Elements,
        mut commands: Commands,
    ) {
        commands
            .entity(query.single())
            .remove::<MainMenuBackground>();
    }
}

/// Backgrounds for the main menu
///
/// TODO: Add backgrounds
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BackgroundEnum {
    /// 3D cubemaps
    CubeMap(CubeMapBackground),
    /// 2D images
    Image(ImageBackground),
    /// Solid colors
    Solid(ColorBackground),
}

impl Default for BackgroundEnum {
    fn default() -> Self { Self::CubeMap(CubeMapBackground::default()) }
}
