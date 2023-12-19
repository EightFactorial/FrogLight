use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod character;
pub mod crafting;
pub mod inventory;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct PlayerNodeComponent;

impl MenuComponent for PlayerNodeComponent {
    fn setup(app: &mut App) {
        inventory::InventoryNodeComponent::setup(app);
        character::CharacterNodeComponent::setup(app);
        crafting::CraftingNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building PlayerNodeComponent");
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
            .spawn((PlayerNodeComponent, node))
            .set_parent(parent)
            .id();
        inventory::InventoryNodeComponent::build(entity, world);
        character::CharacterNodeComponent::build(entity, world);
        crafting::CraftingNodeComponent::build(entity, world);
    }
}
