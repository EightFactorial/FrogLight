use belly::prelude::Elements;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::systems::app_state::{ApplicationState, InMenuSet};

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
        (
            MainMenuBackground::show.run_if(any_with_component::<MainMenuBackground>()),
            BackgroundColorEnum::create.run_if(not(any_with_component::<BackgroundColor>())),
            BackgroundImageEnum::create.run_if(not(any_with_component::<BackgroundImage>())),
        )
            .in_set(InMenuSet),
    );

    app.add_systems(
        Update,
        BackgroundImageEnum::on_window_resize
            .run_if(any_with_component::<BackgroundImage>())
            .in_set(InMenuSet),
    );

    app.add_systems(
        OnExit(ApplicationState::InMenu),
        (
            BackgroundColorEnum::destroy.run_if(any_with_component::<BackgroundColor>()),
            BackgroundImageEnum::destroy.run_if(any_with_component::<BackgroundImage>()),
        )
            .in_set(InMenuSet),
    );
}

/// A resource to hold the main menu asset handles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct BackgroundAssets(pub Vec<HandleUntyped>);

/// A marker component for the main menu background
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuBackground;

impl MainMenuBackground {
    fn show(mut elements: Elements) {
        elements
            .select(".root div.main-background")
            .remove_class("hidden");
    }
}

/// Backgrounds for the main menu
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackgroundEnum {
    /// 3D cubemaps
    CubeMap(BackgroundCubeMapEnum),
    /// 2D images
    Image(BackgroundImageEnum),
    /// Solid colors
    Color(BackgroundColorEnum),
}

impl Default for BackgroundEnum {
    fn default() -> Self { Self::CubeMap(BackgroundCubeMapEnum::default()) }
}
