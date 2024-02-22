use bevy::prelude::*;

use super::systemset::LoadingScreenPostStartupSet;
use crate::default_camera::default_camera2d_bundle;

/// A plugin that adds a loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceLoadingScreenPlugin;

impl Plugin for InterfaceLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        // Build `SystemSet`s
        crate::menus::build(app);
        super::systemset::build(app);

        // Add systems and components
        super::build(app);

        // Create a Camera2d if one does not exist by `PostStartup`
        app.add_systems(
            PostStartup,
            Self::create_camera2d
                .run_if(not(any_with_component::<Camera2d>))
                .run_if(run_once())
                .in_set(LoadingScreenPostStartupSet),
        );
    }
}

impl InterfaceLoadingScreenPlugin {
    fn create_camera2d(mut commands: Commands) {
        debug!("Creating Camera2d");
        commands.spawn(default_camera2d_bundle());
    }
}
