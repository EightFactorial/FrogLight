use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod background;
pub mod buttons;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct PauseNodeComponent;

impl MenuComponent for PauseNodeComponent {
    fn setup(app: &mut App) {
        buttons::ButtonsNodeComponent::setup(app);
        background::BackgroundNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building PauseNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let entity = world
            .spawn((PauseNodeComponent, node))
            .set_parent(parent)
            .id();
        buttons::ButtonsNodeComponent::build(entity, world);
        background::BackgroundNodeComponent::build(entity, world);
    }
}
