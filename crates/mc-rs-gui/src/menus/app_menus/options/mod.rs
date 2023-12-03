use bevy::prelude::*;

use crate::menus::{
    app_menus::states::MainMenuState,
    traits::{InState, MenuComponent},
};

pub mod background;
pub mod buttons;
pub mod menus;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct OptionsNodeComponent;

impl MenuComponent for OptionsNodeComponent {
    fn setup(app: &mut App) {
        menus::MenusNodeComponent::setup(app);
        buttons::ButtonsNodeComponent::setup(app);
        background::BackgroundNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building OptionsNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MainMenuState::Options),
            ..Default::default()
        };
        let entity = world
            .spawn((OptionsNodeComponent, node))
            .set_parent(parent)
            .id();
        menus::MenusNodeComponent::build(entity, world);
        buttons::ButtonsNodeComponent::build(entity, world);
        background::BackgroundNodeComponent::build(entity, world);
    }
}
