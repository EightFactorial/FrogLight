use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

use super::{block_bg::BlockBackground, MainMenuComponent, MainMenuState};

mod message;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct JoiningMenu;

impl MainMenuComponent for JoiningMenu {
    type Background = BlockBackground;
    const STATE: MainMenuState = MainMenuState::Joining;
}

impl MenuComponent for JoiningMenu {
    fn add_systems(app: &mut App) {
        // TODO: Add systems

        <Self as MainMenuComponent>::Background::add_systems(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building JoiningMenu");

        // Spawn MenuComponent
        let entity = world.spawn(Self).id();
        world.entity_mut(parent).add_child(entity);

        // Build background
        <Self as MainMenuComponent>::Background::build(parent, world);
    }
}
