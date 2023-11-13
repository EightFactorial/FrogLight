use bevy::prelude::*;
use mc_rs_core::schedule::{set::MenuSet, state::ApplicationState};

use crate::{
    interface::{
        camera::DefaultCamera, main_menu::cube::BackgroundCube, state::MainMenuState,
        InterfaceAssets,
    },
    traits::{interface::InterfaceComponent, world::MenuVisibility},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuBackground;

impl InterfaceComponent for MainMenuBackground {
    fn setup(app: &mut App) {
        // Spawn or enable a Camera3d when entering the ApplicationState::MainMenu state
        // Show or build the background when entering the ApplicationState::MainMenu state
        app.add_systems(
            OnEnter(ApplicationState::MainMenu),
            (
                (
                    DefaultCamera::custom_fov_camera3d::<75>
                        .run_if(not(any_with_component::<Camera3d>())),
                    DefaultCamera::enable_camera3d.run_if(any_with_component::<Camera3d>()),
                )
                    .run_if(in_state(MainMenuState::Main)),
                MainMenuBackground::show.run_if(any_with_component::<MainMenuBackground>()),
                MainMenuBackground::build.run_if(not(any_with_component::<MainMenuBackground>())),
            )
                .in_set(MenuSet),
        );

        // Destroy the Camera3d when exiting the ApplicationState::MainMenu state
        // Destroy the background when exiting the ApplicationState::MainMenu state
        app.add_systems(
            OnExit(ApplicationState::MainMenu),
            (
                MainMenuBackground::destroy.run_if(any_with_component::<MainMenuBackground>()),
                DefaultCamera::destroy_camera3d.run_if(any_with_component::<Camera3d>()),
            )
                .in_set(MenuSet),
        );

        // Show the background when entering the MainMenuState::Main state
        app.add_systems(
            OnEnter(MainMenuState::Main),
            (
                MainMenuBackground::show.run_if(any_with_component::<MainMenuBackground>()),
                DefaultCamera::enable_camera3d.run_if(any_with_component::<Camera3d>()),
            )
                .run_if(in_state(ApplicationState::MainMenu))
                .in_set(MenuSet),
        );

        // Hide the background when exiting the MainMenuState::Main state
        app.add_systems(
            OnExit(MainMenuState::Main),
            (
                MainMenuBackground::hide.run_if(any_with_component::<MainMenuBackground>()),
                DefaultCamera::disable_camera3d.run_if(any_with_component::<Camera3d>()),
            )
                .run_if(in_state(ApplicationState::MainMenu))
                .in_set(MenuSet),
        );

        // Rotate the background when in the ApplicationState::MainMenu state
        app.add_systems(
            Update,
            MainMenuBackground::rotate
                .run_if(
                    in_state(ApplicationState::MainMenu)
                        .and_then(any_with_component::<MainMenuBackground>()),
                )
                .in_set(MenuSet),
        );
    }

    fn build(_main_menu: Entity, world: &mut World) { MainMenuBackground::build(world); }
}

impl MainMenuBackground {
    /// Build the main menu background.
    fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuBackground");

        // Get the cube mesh
        let mesh = BackgroundCube::create_cube_mesh(world);

        // Get the panorama textures
        let panorama = BackgroundCube::create_cube_texture(world);

        // Create a material
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(panorama),
            unlit: true,
            ..Default::default()
        });

        // Add the mesh and material to the interface assets
        let mut interface_assets = world.resource_mut::<InterfaceAssets>();
        interface_assets.push(mesh.clone_weak().untyped());
        interface_assets.push(material.clone_weak().untyped());

        // Spawn the background
        world.spawn((
            MainMenuBackground,
            PbrBundle {
                mesh,
                material,
                visibility: world
                    .get_menu_visibility(ApplicationState::MainMenu, MainMenuState::Main),
                ..Default::default()
            },
        ));
    }

    /// Rotate the main menu background.
    fn rotate(mut query: Query<&mut Transform, With<MainMenuBackground>>, time: Res<Time<Real>>) {
        let delta = time.delta_seconds();

        query.for_each_mut(|mut transform| {
            transform.rotate(Quat::from_rotation_y(delta / 100.));
        });
    }

    fn show(mut query: Query<&mut Visibility, With<MainMenuBackground>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing MainMenuBackground");

        query.for_each_mut(|mut vis| {
            *vis = Visibility::Visible;
        });
    }

    fn hide(mut query: Query<&mut Visibility, With<MainMenuBackground>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding MainMenuBackground");

        query.for_each_mut(|mut vis| {
            *vis = Visibility::Hidden;
        });
    }

    fn destroy(mut query: Query<Entity, With<MainMenuBackground>>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Destroying MainMenuBackground");

        query.for_each_mut(|entity| {
            commands.entity(entity).despawn_recursive();
        });
    }
}
