//! A plugin that adds the ability to toggle the inspector with a keybind.

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod enable;
pub use enable::InspectorEnable;

pub(crate) mod systemset;
use systemset::InspectorUpdateSet;

/// A plugin that adds the ability to toggle the inspector with a keybind.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        systemset::build(app);

        // Add the `InspectorEnable` resource
        app.register_type::<InspectorEnable>().init_resource::<InspectorEnable>();

        // Add the system to toggle the inspector
        app.add_systems(
            Update,
            InspectorEnable::inspector_keybind
                .run_if(resource_exists::<InspectorEnable>)
                .in_set(InspectorUpdateSet),
        );

        // Create the `WorldInspectorPlugin` plugin
        let mut inspector = WorldInspectorPlugin::new();
        // Add the condition to check if the world inspector should be enabled
        inspector = inspector.run_if(InspectorEnable::is_inspector_enabled);

        // Add the world inspector plugin
        app.add_plugins(inspector);
    }
}
