use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct LoadingBar;

impl MenuComponent for LoadingBar {
    fn setup(_app: &mut App) {}

    fn build(_parent: Entity, _world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building LoadingBar");

        // TODO
    }
}
