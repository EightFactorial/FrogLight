use bevy::prelude::*;

use crate::networking::network::LocalPlayer;

use super::{
    app_state::{ApplicationState, GameSet},
    settings::Settings,
};

pub mod components;

pub(super) fn add_systems(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::InGame),
        create_camera
            .run_if(not(any_with_component::<Camera3d>()))
            .in_set(GameSet),
    );
}

fn create_camera(player: Res<LocalPlayer>, settings: Res<Settings>, mut commands: Commands) {
    debug!("Creating camera for {:?}", *player);

    commands.entity(player.head).insert((
        Camera3dBundle::default(),
        PerspectiveProjection {
            fov: settings.game.camera.fov,
            ..Default::default()
        },
    ));
}
