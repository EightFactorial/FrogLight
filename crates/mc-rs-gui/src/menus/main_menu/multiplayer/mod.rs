use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

use super::{block_bg::BlockBackground, MainMenuComponent, MainMenuState};

mod list;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MultiplayerMenu;

impl MainMenuComponent for MultiplayerMenu {
    type Background = BlockBackground;
    const STATE: MainMenuState = MainMenuState::Multiplayer;
}

impl MenuComponent for MultiplayerMenu {
    fn add_systems(app: &mut App) {
        // TODO: Add systems

        <Self as MainMenuComponent>::Background::add_systems(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MultiplayerMenu");

        // Spawn MenuComponent
        let entity = world.spawn(Self).id();
        world.entity_mut(parent).add_child(entity);

        // Build background
        <Self as MainMenuComponent>::Background::build(parent, world);
    }
}
