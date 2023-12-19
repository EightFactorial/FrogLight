use bevy::prelude::*;

use crate::{
    menus::{
        app_menus::states::MainMenuState,
        states::menus::{MenuComponentMenusSet, MenuComponentState},
        traits::{InState, MenuComponent},
    },
    resources::scale::GuiScaleComponent,
};

pub mod background;
pub mod buttons;
pub mod servers;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MultiplayerNodeComponent;

impl MultiplayerNodeComponent {
    const MENU_WIDTH: u32 = 240;
    const MENU_HEIGHT: u32 = 180;
}

impl MenuComponent for MultiplayerNodeComponent {
    fn setup(app: &mut App) {
        app.add_systems(
            OnEnter(MainMenuState::Multiplayer),
            Self::show
                .run_if(in_state(MenuComponentState::Menus))
                .in_set(MenuComponentMenusSet),
        );
        app.add_systems(
            OnExit(MainMenuState::Multiplayer),
            Self::hide
                .run_if(in_state(MenuComponentState::Menus))
                .in_set(MenuComponentMenusSet),
        );

        buttons::ButtonsNodeComponent::setup(app);
        background::BackgroundNodeComponent::setup(app);
        servers::ServersNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MultiplayerNodeComponent");

        let node = world
            .spawn((
                MultiplayerNodeComponent,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    visibility: world.get_visibility(MainMenuState::Multiplayer),
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        let centered = world
            .spawn((
                MultiplayerCenterNodeComponent,
                GuiScaleComponent::new(Self::MENU_WIDTH, Self::MENU_HEIGHT),
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .set_parent(node)
            .id();

        buttons::ButtonsNodeComponent::build(centered, world);
        background::BackgroundNodeComponent::build(centered, world);
        servers::ServersNodeComponent::build(centered, world);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MultiplayerCenterNodeComponent;
