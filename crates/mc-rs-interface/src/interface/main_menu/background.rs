use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct MainMenuBackground;

impl MainMenuBackground {
    pub(super) fn setup(_app: &mut App) {
        // TODO: Add systems
    }

    pub(super) fn build(main_menu: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuBackground");

        let background = world.spawn(MainMenuBackground);

        // TODO: Build main menu background

        let background = background.id();
        world.entity_mut(main_menu).add_child(background);
    }
}
