use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    menus::MenuRoot,
    systems::{
        settings::Settings,
        states::{
            application::MenuSet,
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
            BackgroundRoot::destroy.run_if(
                resource_changed::<Settings>().and_then(any_with_component::<BackgroundRoot>()),
            ),
            BackgroundRoot::create.run_if(not(any_with_component::<BackgroundRoot>())),
        )
            .chain()
            .in_set(MenuSet),
    );

    app.add_systems(
        OnEnter(MenuState::Main),
        BackgroundRoot::show
            .run_if(any_with_component::<BackgroundRoot>())
            .in_set(MenuMainSet),
    );
    app.add_systems(
        OnExit(MenuState::Main),
        BackgroundRoot::hide
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
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.),
                        top: Val::Px(0.),
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..Default::default()
                    },
                    visibility,
                    background_color: Color::BLUE.with_a(0.1).into(),
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
    pub fn show(mut vis: Query<&mut Visibility, With<BackgroundRoot>>) {
        *vis.single_mut() = Visibility::Visible;
    }

    /// Make the background visible
    pub fn hide(mut vis: Query<&mut Visibility, With<BackgroundRoot>>) {
        *vis.single_mut() = Visibility::Hidden;
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
