use bevy::prelude::*;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::DebugUpdateSet;
#[cfg(feature = "inspector")]
use crate::InspectorEnable;

/// The [`Plugin`] for the [`froglight-debug`](crate) crate.
///
/// Adds debug menus and utilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(feature = "inspector")]
pub struct DebugPlugin {
    /// Set to `true` to enable the Bevy World Inspector.
    pub inspector: bool,
}

/// The [`Plugin`] for the [`froglight-debug`](crate) crate.
///
/// Adds debug menus and utilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(not(feature = "inspector"))]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, DebugUpdateSet);

        #[cfg(feature = "inspector")]
        if self.inspector {
            info!("World Inspector enabled, press F3 + I to toggle");

            // Add the InspectorEnable resource
            crate::InspectorEnable::build(app);

            // Add the Bevy World Inspector plugin
            app.add_plugins(WorldInspectorPlugin::new().run_if(InspectorEnable::is_enabled));
        }
    }
}
