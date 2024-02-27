use bevy::prelude::*;
use froglight_core::resources::MainMenuEnable;

use super::MainMenuBackground;
use crate::{
    default_camera::default_camera3d_bundle, menus::mainmenu::systemset::MainMenuUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<MainMenuBackgroundCamera>();
    app.add_systems(
        Update,
        MainMenuBackgroundCamera::background_camera
            .run_if(resource_exists_and_changed::<MainMenuEnable>)
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
        res: Res<MainMenuEnable>,
        mut commands: Commands,
    ) {
        if **res {
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

    pub(crate) fn bundle() -> impl Bundle {
        (
            Name::new("MainMenuBackgroundCamera"),
            MainMenuBackground::RENDER_LAYER,
            MainMenuBackgroundCamera,
            Camera3dBundle {
                transform: Transform::from_rotation(Quat::from_rotation_x(-7.5f32.to_radians())),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90f32.to_radians(),
                    ..Default::default()
                }),
                ..default_camera3d_bundle()
            },
        )
    }
}
