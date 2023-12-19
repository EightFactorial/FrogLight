use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod chest;
pub mod furnace;
pub mod miscellaneous;
pub mod player;
pub mod table;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct InventoryNodeComponent;

impl MenuComponent for InventoryNodeComponent {
    fn setup(app: &mut App) {
        chest::ChestNodeComponent::setup(app);
        player::PlayerNodeComponent::setup(app);
        table::TableNodeComponent::setup(app);
        furnace::FurnaceNodeComponent::setup(app);
        miscellaneous::MiscellaneousNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building InventoryNodeComponent");
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
            .spawn((InventoryNodeComponent, node))
            .set_parent(parent)
            .id();
        chest::ChestNodeComponent::build(entity, world);
        player::PlayerNodeComponent::build(entity, world);
        table::TableNodeComponent::build(entity, world);
        furnace::FurnaceNodeComponent::build(entity, world);
        miscellaneous::MiscellaneousNodeComponent::build(entity, world);
    }
}
