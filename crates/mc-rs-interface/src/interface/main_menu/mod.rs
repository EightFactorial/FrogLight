use bevy::prelude::*;

mod background;
use background::MainMenuBackground;

use crate::traits::interface::SubInterface;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuInterface;

impl SubInterface for MainMenuInterface {
    fn setup(app: &mut App) {
        // TODO: Add systems

        MainMenuBackground::setup(app);
    }

    fn build(root: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuInterface");

        let main_menu = world.spawn(MainMenuInterface);

        // TODO: Build main menu interface

        let main_menu = main_menu.id();
        world.entity_mut(root).add_child(main_menu);

        MainMenuBackground::build(main_menu, world);
    }
}
