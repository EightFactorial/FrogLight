use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod double;
pub mod ender;
pub mod shulker;
pub mod single;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ChestNodeComponent;

impl MenuComponent for ChestNodeComponent {
    fn setup(app: &mut App) {
        single::SingleNodeComponent::setup(app);
        double::DoubleNodeComponent::setup(app);
        shulker::ShulkerNodeComponent::setup(app);
        ender::EnderNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building ChestNodeComponent");
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
            .spawn((ChestNodeComponent, node))
            .set_parent(parent)
            .id();
        single::SingleNodeComponent::build(entity, world);
        double::DoubleNodeComponent::build(entity, world);
        shulker::ShulkerNodeComponent::build(entity, world);
        ender::EnderNodeComponent::build(entity, world);
    }
}
