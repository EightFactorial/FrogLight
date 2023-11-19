use bevy::prelude::*;

use crate::menus::traits::{MenuComponent, VisibilityFromWorld};

use super::{MainMenuComponent, MainMenuState};

mod background;
use background::CubemapBackground;

mod buttons;
use buttons::MainMenuButtons;

mod title;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenu;

impl MainMenuComponent for MainMenu {
    type Background = CubemapBackground;
    const STATE: MainMenuState = MainMenuState::Main;
}

impl MenuComponent for MainMenu {
    fn setup(app: &mut App) {
        app.add_systems(OnEnter(<Self as MainMenuComponent>::STATE), Self::show);
        app.add_systems(OnExit(<Self as MainMenuComponent>::STATE), Self::hide);

        <Self as MainMenuComponent>::Background::setup(app);
        MainMenuButtons::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenu");

        // Create node
        let node = NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            visibility: world.get_visibility(MainMenuState::Main),
            ..Default::default()
        };

        // Spawn MenuComponent
        let entity = world.spawn((MainMenu, node)).set_parent(parent).id();

        // Build components
        <Self as MainMenuComponent>::Background::build(parent, world);
        MainMenuButtons::build(entity, world);
    }
}
