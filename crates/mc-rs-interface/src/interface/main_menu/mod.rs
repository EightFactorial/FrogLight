use bevy::prelude::*;
use mc_rs_core::{
    schedule::{set::MenuSet, state::ApplicationState},
    ResourceLocation,
};

use crate::resourcepacks::{ResourcePackAsset, ResourcePacks};

use super::{state::MainMenuState, InterfaceRoot};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuInterface;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuBackgroundCube;

impl MainMenuInterface {
    pub(super) fn setup(app: &mut App) {
        app.add_systems(
            OnEnter(ApplicationState::MainMenu),
            (
                InterfaceRoot::default_camera3d.run_if(InterfaceRoot::no_camera3d),
                Self::show_cube.run_if(not(Self::is_no_cube)),
                Self::spawn_background_cube.run_if(Self::is_no_cube),
            )
                .run_if(Self::show_main_menu)
                .in_set(MenuSet),
        );

        app.add_systems(
            Update,
            (
                Self::hide_cube.run_if(not(Self::show_main_menu)),
                Self::rotate_cube.run_if(any_with_component::<MainMenuBackgroundCube>()),
            )
                .in_set(MenuSet),
        );
    }

    fn show_main_menu(
        app_state: Res<State<ApplicationState>>,
        menu_state: Res<State<MainMenuState>>,
    ) -> bool {
        matches!(**app_state, ApplicationState::MainMenu)
            && matches!(**menu_state, MainMenuState::Main)
    }

    fn is_no_cube(query: Query<(), With<MainMenuBackgroundCube>>) -> bool { query.is_empty() }

    fn spawn_background_cube(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        // mut images: ResMut<Assets<Image>>,
        mut materials: ResMut<Assets<StandardMaterial>>,

        packs: Res<ResourcePacks>,
        assets: Res<Assets<ResourcePackAsset>>,
    ) {
        let mesh = meshes.add(Mesh::from(shape::Cube { size: -1.0 }));

        let texture_0 = packs.get_texture(
            &ResourceLocation::new("minecraft:gui/title/background/panorama_0"),
            &assets,
        );

        let material = materials.add(StandardMaterial {
            base_color_texture: Some(texture_0.clone()),
            unlit: true,
            ..Default::default()
        });

        commands.spawn((
            MainMenuBackgroundCube,
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                visibility: Visibility::Visible,
                ..Default::default()
            },
        ));
    }

    fn rotate_cube(
        time: Res<Time<Real>>,
        mut query: Query<&mut Transform, With<MainMenuBackgroundCube>>,
    ) {
        query.iter_mut().for_each(|mut transform| {
            transform.rotate(Quat::from_axis_angle(Vec3::Y, time.delta_seconds() / 100.))
        });
    }

    fn hide_cube(mut query: Query<&mut Visibility, With<MainMenuBackgroundCube>>) {
        query
            .iter_mut()
            .for_each(|mut vis| *vis = Visibility::Hidden);
    }

    fn show_cube(mut query: Query<&mut Visibility, With<MainMenuBackgroundCube>>) {
        query
            .iter_mut()
            .for_each(|mut vis| *vis = Visibility::Visible);
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn spawn(
        root: Res<InterfaceRoot>,
        app_state: Res<State<ApplicationState>>,
        menu_state: Res<State<MainMenuState>>,

        _packs: Res<ResourcePacks>,
        _textures: Res<Assets<ResourcePackAsset>>,
        mut _images: ResMut<Assets<Image>>,

        _assets: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let visibility = match (**app_state, **menu_state) {
            (ApplicationState::MainMenu, MainMenuState::Main) => Visibility::Visible,
            _ => Visibility::Hidden,
        };

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning MainMenuInterface with visibility: {visibility:?}");

        let main_menu = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            visibility,
            background_color: Color::NONE.into(),
            ..Default::default()
        };

        commands.entity(**root).with_children(|root| {
            root.spawn((main_menu, Self));
        });
    }
}
