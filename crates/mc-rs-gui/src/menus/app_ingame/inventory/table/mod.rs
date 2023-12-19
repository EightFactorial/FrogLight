use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod cartography;
pub mod crafting;
pub mod enchanting;
pub mod fleching;
pub mod loom;
pub mod smithing;
pub mod stonecutting;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TableNodeComponent;

impl MenuComponent for TableNodeComponent {
    fn setup(app: &mut App) {
        smithing::SmithingNodeComponent::setup(app);
        stonecutting::StonecuttingNodeComponent::setup(app);
        cartography::CartographyNodeComponent::setup(app);
        crafting::CraftingNodeComponent::setup(app);
        enchanting::EnchantingNodeComponent::setup(app);
        fleching::FlechingNodeComponent::setup(app);
        loom::LoomNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building TableNodeComponent");
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
            .spawn((TableNodeComponent, node))
            .set_parent(parent)
            .id();
        smithing::SmithingNodeComponent::build(entity, world);
        stonecutting::StonecuttingNodeComponent::build(entity, world);
        cartography::CartographyNodeComponent::build(entity, world);
        crafting::CraftingNodeComponent::build(entity, world);
        enchanting::EnchantingNodeComponent::build(entity, world);
        fleching::FlechingNodeComponent::build(entity, world);
        loom::LoomNodeComponent::build(entity, world);
    }
}
