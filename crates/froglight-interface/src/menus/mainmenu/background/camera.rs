use bevy::prelude::*;
use froglight_core::resources::MainMenuEnable;

use super::{MainMenuBackground, MainMenuBackgroundEnable};
use crate::{
    default_camera::default_camera3d_bundle,
    menus::{mainmenu::systemset::MainMenuUpdateSet, InterfaceMenuState},
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<MainMenuBackgroundCamera>();
    app.add_systems(
        Update,
        MainMenuBackgroundCamera::background_camera
            .run_if(
                resource_exists_and_changed::<MainMenuEnable>
                    .or_else(resource_exists_and_changed::<MainMenuBackgroundEnable>),
            )
            .in_set(MainMenuUpdateSet),
    );
}

/// A marker [`Component`] for the main menu background camera.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuBackgroundCamera;

impl MainMenuBackgroundCamera {
    fn background_camera(
        query: Query<Entity, With<Self>>,
        enable: Res<MainMenuEnable>,

        state: Res<State<InterfaceMenuState>>,
        state_enable: Res<MainMenuBackgroundEnable>,

        mut commands: Commands,
    ) {
        if **enable && state_enable.is_enabled_in(**state) {
            if query.iter().count() == 0 {
                debug!("Creating MainMenuBackgroundCamera");
                commands.spawn(Self::bundle());
            }
        } else {
            for entity in &query {
                debug!("Deleting MainMenuBackgroundCamera");
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    fn bundle() -> impl Bundle {
        (
            Name::new("MainMenuBackgroundCamera"),
            MainMenuBackground::RENDER_LAYER,
            MainMenuBackgroundCamera,
            Camera3dBundle {
                // Angle the camera slightly down
                transform: Transform::from_rotation(Quat::from_rotation_x(-7.5f32.to_radians())),
                // Set the camera fov to 90 degrees
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90f32.to_radians(),
                    ..Default::default()
                }),
                ..default_camera3d_bundle()
            },
        )
    }
}
