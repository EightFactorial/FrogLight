use bevy::prelude::*;

/// A [`Plugin`] that manages menus and other GUI elements
///
/// By default, this also adds the
/// [`LoadingPlugin`](froglight_loading::LoadingPlugin)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Build plugin

        // Add the loading screen plugin if the default-loading feature is enabled
        #[cfg(feature = "default-loading")]
        app.add_plugins(froglight_loading::LoadingPlugin);
    }
}
