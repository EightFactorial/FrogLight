use bevy::prelude::*;

use crate::menus::{
    app_menus::states::MainMenuState,
    traits::{InState, MenuComponent},
};

pub mod background;
pub mod buttons;
pub mod servers;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MultiplayerNodeComponent;

impl MenuComponent for MultiplayerNodeComponent {
    fn setup(app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::Multiplayer), Self::show);
        app.add_systems(OnExit(MainMenuState::Multiplayer), Self::hide);

        buttons::ButtonsNodeComponent::setup(app);
        background::BackgroundNodeComponent::setup(app);
        servers::ServersNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MultiplayerNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MainMenuState::Multiplayer),
            ..Default::default()
        };
        let entity = world
            .spawn((MultiplayerNodeComponent, node))
            .set_parent(parent)
            .id();
        buttons::ButtonsNodeComponent::build(entity, world);
        background::BackgroundNodeComponent::build(entity, world);
        servers::ServersNodeComponent::build(entity, world);
    }
}
