use bevy::prelude::*;

use crate::menus::{
    states::menus::MenuComponentState,
    traits::{InState, MenuComponent},
};

use self::states::MainMenuState;

pub mod main_menu;
pub mod multiplayer;
pub mod options;
pub mod states;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct AppMenusNodeComponent;

impl MenuComponent for AppMenusNodeComponent {
    fn setup(app: &mut App) {
        app.add_state::<MainMenuState>();

        options::OptionsNodeComponent::setup(app);
        multiplayer::MultiplayerNodeComponent::setup(app);
        main_menu::MainMenuNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building AppMenusNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MenuComponentState::Menus),
            ..Default::default()
        };

        let entity = world
            .spawn((AppMenusNodeComponent, node))
            .set_parent(parent)
            .id();

        options::OptionsNodeComponent::build(entity, world);
        multiplayer::MultiplayerNodeComponent::build(entity, world);
        main_menu::MainMenuNodeComponent::build(entity, world);
    }
}
