use bevy::prelude::*;

use crate::menus::{
    states::menus::MenuComponentState,
    traits::{InState, MenuComponent},
};

pub mod hud;
pub mod inventory;
pub mod pause;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct AppIngameNodeComponent;

impl MenuComponent for AppIngameNodeComponent {
    fn setup(app: &mut App) {
        inventory::InventoryNodeComponent::setup(app);
        hud::HudNodeComponent::setup(app);
        pause::PauseNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building AppIngameNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MenuComponentState::InGame),
            ..Default::default()
        };
        let entity = world
            .spawn((AppIngameNodeComponent, node))
            .set_parent(parent)
            .id();
        inventory::InventoryNodeComponent::build(entity, world);
        hud::HudNodeComponent::build(entity, world);
        pause::PauseNodeComponent::build(entity, world);
    }
}
