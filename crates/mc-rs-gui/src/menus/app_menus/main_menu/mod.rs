use bevy::prelude::*;

use crate::menus::{
    app_menus::states::MainMenuState,
    traits::{InState, MenuComponent},
};

pub mod background;
pub mod buttons;
pub mod messages;
pub mod title;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuNodeComponent;

impl MenuComponent for MainMenuNodeComponent {
    fn setup(app: &mut App) {
        buttons::ButtonsNodeComponent::setup(app);
        background::BackgroundNodeComponent::setup(app);
        title::TitleNodeComponent::setup(app);
        messages::MessagesNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuNodeComponent");
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
            visibility: world.get_visibility(MainMenuState::MainMenu),
            ..Default::default()
        };
        let entity = world
            .spawn((MainMenuNodeComponent, node))
            .set_parent(parent)
            .id();
        buttons::ButtonsNodeComponent::build(entity, world);
        background::BackgroundNodeComponent::build(entity, world);
        title::TitleNodeComponent::build(entity, world);
        messages::MessagesNodeComponent::build(entity, world);
    }
}
