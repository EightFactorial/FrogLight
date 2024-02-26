use bevy::prelude::*;
use froglight_core::resources::MainMenuEnable;

use crate::{
    default_camera::default_camera3d_bundle, menus::mainmenu::systemset::MainMenuUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_systems(
        Update,
        (
            MainMenuBackgroundCamera::create_camera_when_enabled
                .run_if(not(any_with_component::<MainMenuBackgroundCamera>))
                .run_if(resource_equals(MainMenuEnable(true))),
            MainMenuBackgroundCamera::delete_camera_when_disabled
                .run_if(resource_equals(MainMenuEnable(false))),
        )
            .run_if(resource_exists_and_changed::<MainMenuEnable>)
            .in_set(MainMenuUpdateSet),
    );
}

/// A marker [`Component`] for the main menu background camera.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuBackgroundCamera;

impl MainMenuBackgroundCamera {
    fn create_camera_when_enabled(mut commands: Commands) {
        debug!("Creating MainMenuBackgroundCamera");
        commands.spawn(Self::create());
    }

    fn delete_camera_when_disabled(query: Query<Entity, With<Self>>, mut commands: Commands) {
        debug!("Deleting MainMenuBackgroundCamera");
        for entity in &query {
            commands.entity(entity).despawn_recursive();
        }
    }

    pub(crate) fn create() -> impl Bundle {
        let mut camera = default_camera3d_bundle();
        camera.camera.clear_color = ClearColorConfig::Custom(Color::MIDNIGHT_BLUE);
        camera.projection = Projection::Perspective(PerspectiveProjection {
            fov: 90f32.to_radians(),
            ..Default::default()
        });

        (Self, Name::new("MainMenuBackgroundCamera"), camera)
    }
}
