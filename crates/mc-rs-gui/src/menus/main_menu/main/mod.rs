use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

use super::{MainMenuComponent, MainMenuState};

mod background;
use background::CubemapBackground;

mod title;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenu;

impl MainMenuComponent for MainMenu {
    type Background = CubemapBackground;
    const STATE: MainMenuState = MainMenuState::Main;
}

impl MenuComponent for MainMenu {
    fn add_systems(app: &mut App) {
        // TODO: Add systems

        <Self as MainMenuComponent>::Background::add_systems(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenu");

        // Spawn MenuComponent
        let entity = world.spawn(Self).id();
        world.entity_mut(parent).add_child(entity);

        // Build background
        <Self as MainMenuComponent>::Background::build(parent, world);
    }
}
