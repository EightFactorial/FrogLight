use bevy::prelude::*;

/// A [`Plugin`] that manages menus and other GUI elements
///
/// By default, this also adds the
/// [`LoadingPlugin`](froglight_loading::LoadingPlugin)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, _app: &mut App) {}
}
