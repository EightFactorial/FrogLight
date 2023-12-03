use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod experience;
pub mod inventory;
pub mod status;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct HotbarNodeComponent;

impl MenuComponent for HotbarNodeComponent {
    fn setup(app: &mut App) {
        status::StatusNodeComponent::setup(app);
        inventory::InventoryNodeComponent::setup(app);
        experience::ExperienceNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building HotbarNodeComponent");
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
            .spawn((HotbarNodeComponent, node))
            .set_parent(parent)
            .id();
        status::StatusNodeComponent::build(entity, world);
        inventory::InventoryNodeComponent::build(entity, world);
        experience::ExperienceNodeComponent::build(entity, world);
    }
}
