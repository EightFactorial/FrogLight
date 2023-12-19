use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod left;
pub mod right;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MessagesNodeComponent;

impl MenuComponent for MessagesNodeComponent {
    fn setup(app: &mut App) {
        right::RightNodeComponent::setup(app);
        left::LeftNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building MessagesNodeComponent");
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
            .spawn((MessagesNodeComponent, node))
            .set_parent(parent)
            .id();
        right::RightNodeComponent::build(entity, world);
        left::LeftNodeComponent::build(entity, world);
    }
}
