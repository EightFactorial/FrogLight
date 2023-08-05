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

use self::{
    cubemap::BackgroundCubeMapEnum,
    image::{BackgroundImage, BackgroundImageEnum},
    solid::BackgroundColorEnum,
};

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
        Update,
        MainMenuBackground::destroy
            .run_if(
                any_with_component::<MainMenuBackground>().and_then(resource_changed::<Settings>()),
            )
            .before(MainMenuBackground::create)
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

/// A resource to hold the main menu asset handles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct BackgroundAssets(pub Vec<HandleUntyped>);

impl MainMenuBackground {
    /// Create the background
    fn create(
        root: Res<MenuRoot>,
        settings: Res<Settings>,
        assets: Res<AssetServer>,
        elements: Elements,
        mut commands: Commands,
    ) {
        commands.entity(**root).insert(MainMenuBackground);

        match &settings.menu.main_menu {
            BackgroundEnum::CubeMap(bg) => bg.create(&root, &assets, elements, commands),
            BackgroundEnum::Image(bg) => bg.create(&root, &assets, elements, commands),
            BackgroundEnum::Solid(bg) => bg.create(elements),
        }
    }

    /// Destroy the background
    fn destroy(
        root: Res<MenuRoot>,
        camera: Option<Res<BackgroundCamera>>,
        mut elements: Elements,
        mut commands: Commands,
        mut local: Local<bool>,
    ) {
        // Prevent first run
        if !*local {
            *local = true;
            return;
        }

        if let Some(camera) = camera {
            commands.entity(**camera).despawn_recursive();
        }

        commands.remove_resource::<BackgroundCamera>();
        commands.remove_resource::<BackgroundAssets>();

        commands.entity(**root).remove::<MainMenuBackground>();
        commands.entity(**root).remove::<BackgroundImage>();
        elements.select(".root div.main-background").remove();
    }
}

/// Backgrounds for the main menu
///
/// TODO: Add backgrounds
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BackgroundEnum {
    /// 3D cubemaps
    CubeMap(BackgroundCubeMapEnum),
    /// 2D images
    Image(BackgroundImageEnum),
    /// Solid colors
    Solid(BackgroundColorEnum),
}

impl Default for BackgroundEnum {
    fn default() -> Self { Self::CubeMap(BackgroundCubeMapEnum::default()) }
}
