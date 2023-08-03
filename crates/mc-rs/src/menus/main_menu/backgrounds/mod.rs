use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    menus::MenuRoot,
    systems::{
        settings::Settings,
        states::{
            application::InMenuSet,
            menu::{MenuMainSet, MenuState},
        },
    },
};

use self::{cubemap::CubeMapBackground, image::ImageBackground, solid::ColorBackground};

pub mod cubemap;
pub mod image;
pub mod solid;

pub(super) fn setup_backgrounds(app: &mut App) {
    app.add_systems(
        Update,
        (
            BackgroundRoot::destroy.run_if(resource_changed::<Settings>()),
            BackgroundRoot::create.run_if(not(any_with_component::<BackgroundRoot>())),
        )
            .chain()
            .in_set(InMenuSet),
    );

    app.add_systems(
        OnEnter(MenuState::Main),
        BackgroundRoot::show
            .run_if(any_with_component::<BackgroundRoot>())
            .in_set(MenuMainSet),
    );
}

/// A marker component for the main menu background
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundRoot;

impl BackgroundRoot {
    /// Create the background
    fn create(
        query: Query<Entity, With<MenuRoot>>,
        settings: Res<Settings>,
        state: Res<State<MenuState>>,
        mut commands: Commands,
    ) {
        let visibility = if **state == MenuState::Main {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        let entity = commands
            .spawn((
                BackgroundRoot,
                VisibilityBundle {
                    visibility,
                    ..Default::default()
                },
            ))
            .id();
        commands.entity(query.single()).add_child(entity);

        match &settings.menu.main_menu {
            MainMenuBackground::CubeMap(bg) => bg.create(entity, commands),
            MainMenuBackground::Image(bg) => bg.create(entity, commands),
            MainMenuBackground::Solid(bg) => bg.create(entity, commands),
        }
    }

    /// Destroy the background
    fn destroy(query: Query<Entity, With<BackgroundRoot>>, mut commands: Commands) {
        commands.entity(query.single()).despawn_recursive();
    }

    /// Make the background visible
    pub fn show(mut query: Query<&mut Visibility, With<BackgroundRoot>>) {
        if let Some(mut visibility) = query.iter_mut().next() {
            *visibility = Visibility::Visible;
        }
    }
}

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

impl Default for MainMenuBackground {
    fn default() -> Self { Self::CubeMap(CubeMapBackground::default()) }
}
