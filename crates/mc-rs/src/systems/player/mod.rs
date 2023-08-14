use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::networking::network::LocalPlayer;

use super::{
    app_state::{ApplicationState, GameSet},
    settings::Settings,
};

pub mod input;
pub mod resources;

pub(super) fn add_systems(app: &mut App) {
    resources::add_systems(app);
    input::add_systems(app);

    app.add_systems(
        OnEnter(ApplicationState::Game),
        (
            create_camera.run_if(not(any_with_component::<Camera3d>())),
            clear_background,
            create_light,
        )
            .in_set(GameSet),
    );

    app.add_systems(
        OnExit(ApplicationState::Game),
        (
            destroy_camera.run_if(any_with_component::<Camera3d>()),
            default_background,
            destroy_light,
        )
            .in_set(GameSet),
    );
}

fn create_camera(player: Res<LocalPlayer>, settings: Res<Settings>, mut commands: Commands) {
    debug!("Creating camera for {:?}", *player);

    commands.entity(player.head).insert((
        Camera3dBundle {
            camera: Camera {
                order: isize::MIN,
                ..Default::default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::hex("87CEFA").unwrap()),
                ..Default::default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: settings.game.camera.fov,
                ..Default::default()
            }),
            ..Default::default()
        },
        UiCameraConfig { show_ui: false },
    ));
}

fn destroy_camera(query: Query<Entity, With<Camera3d>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn create_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn destroy_light(query: Query<Entity, With<DirectionalLight>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn clear_background(mut query: Query<&mut Camera2d>) {
    for mut camera in query.iter_mut() {
        camera.clear_color = ClearColorConfig::None;
    }
}

fn default_background(mut query: Query<&mut Camera2d>) {
    for mut camera in query.iter_mut() {
        camera.clear_color = ClearColorConfig::Default;
    }
}
