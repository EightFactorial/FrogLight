use belly::prelude::Elements;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    menus::MenuRoot,
    systems::{
        app_state::{ApplicationState, InMenuSet},
        settings::Settings,
    },
};

use self::{cubemap::CubeMapBackground, image::ImageBackground, solid::ColorBackground};

pub mod cubemap;
pub mod image;
pub mod solid;

pub(super) fn setup_backgrounds(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        MainMenuBackground::create
            .run_if(not(any_with_component::<MainMenuBackground>()))
            .in_set(InMenuSet),
    );

    app.add_systems(
        OnExit(ApplicationState::InMenu),
        MainMenuBackground::destroy
            .run_if(any_with_component::<MainMenuBackground>())
            .in_set(InMenuSet),
    );
}

/// A marker component for the main menu background
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuBackground;

/// A resource to hold the main menu background camera entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
struct BackgroundCamera(pub Entity);

impl MainMenuBackground {
    /// Create the background
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
    fn destroy(
        root: Res<MenuRoot>,
        camera: Option<Res<BackgroundCamera>>,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        commands.entity(**root).remove::<MainMenuBackground>();

        if let Some(camera) = camera {
            commands.entity(**camera).despawn_recursive();
        }

        elements.select(".root div.main-background").remove();
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
