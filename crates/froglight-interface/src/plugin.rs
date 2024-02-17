use bevy::prelude::*;

use crate::UiScalePlugin;

/// A [`Plugin`] that manages menus and other GUI elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        // Add SystemSets
        crate::systemsets::build(app);

        // Add the `UiScalePlugin`
        app.add_plugins(UiScalePlugin);
    }
}
