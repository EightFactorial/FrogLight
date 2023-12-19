use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod advancement;
pub mod notice;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct NotificationsNodeComponent;

impl MenuComponent for NotificationsNodeComponent {
    fn setup(app: &mut App) {
        advancement::AdvancementNodeComponent::setup(app);
        notice::NoticeNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building NotificationsNodeComponent");
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
            .spawn((NotificationsNodeComponent, node))
            .set_parent(parent)
            .id();
        advancement::AdvancementNodeComponent::build(entity, world);
        notice::NoticeNodeComponent::build(entity, world);
    }
}
