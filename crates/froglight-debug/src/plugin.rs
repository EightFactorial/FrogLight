use bevy::prelude::*;

/// The [`Plugin`] for the [`froglight-debug`](crate) crate.
///
/// Adds debug menus and utilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "inspector")]
        {
            use bevy_inspector_egui::quick::WorldInspectorPlugin;
            app.add_plugins(WorldInspectorPlugin::new());
        }
    }
}
