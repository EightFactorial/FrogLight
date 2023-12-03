use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod anvil;
pub mod beacon;
pub mod brewing;
pub mod grindstone;
pub mod hopper;
pub mod horse;
pub mod mule;
pub mod villager;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MiscellaneousNodeComponent;

impl MenuComponent for MiscellaneousNodeComponent {
    fn setup(app: &mut App) {
        brewing::BrewingNodeComponent::setup(app);
        mule::MuleNodeComponent::setup(app);
        grindstone::GrindstoneNodeComponent::setup(app);
        villager::VillagerNodeComponent::setup(app);
        hopper::HopperNodeComponent::setup(app);
        beacon::BeaconNodeComponent::setup(app);
        horse::HorseNodeComponent::setup(app);
        anvil::AnvilNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building MiscellaneousNodeComponent");
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
            .spawn((MiscellaneousNodeComponent, node))
            .set_parent(parent)
            .id();
        brewing::BrewingNodeComponent::build(entity, world);
        mule::MuleNodeComponent::build(entity, world);
        grindstone::GrindstoneNodeComponent::build(entity, world);
        villager::VillagerNodeComponent::build(entity, world);
        hopper::HopperNodeComponent::build(entity, world);
        beacon::BeaconNodeComponent::build(entity, world);
        horse::HorseNodeComponent::build(entity, world);
        anvil::AnvilNodeComponent::build(entity, world);
    }
}
