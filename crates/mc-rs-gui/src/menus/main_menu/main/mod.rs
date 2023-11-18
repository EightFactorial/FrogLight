use bevy::prelude::*;

use crate::menus::traits::{MenuComponent, VisibilityFromWorld};

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
    fn setup(app: &mut App) {
        // TODO: Add systems

        <Self as MainMenuComponent>::Background::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenu");

        // Create node
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MainMenuState::Main),
            ..Default::default()
        };

        // Spawn MenuComponent
        let entity = world.spawn((MainMenu, node)).id();
        world.entity_mut(parent).add_child(entity);

        // Build background
        <Self as MainMenuComponent>::Background::build(parent, world);
    }
}
