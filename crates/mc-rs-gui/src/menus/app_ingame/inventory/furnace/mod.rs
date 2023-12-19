use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod blast;
#[allow(clippy::module_inception)]
pub mod furnace;
pub mod smoker;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct FurnaceNodeComponent;

impl MenuComponent for FurnaceNodeComponent {
    fn setup(app: &mut App) {
        furnace::FurnaceNodeComponent::setup(app);
        smoker::SmokerNodeComponent::setup(app);
        blast::BlastNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building FurnaceNodeComponent");
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
            .spawn((FurnaceNodeComponent, node))
            .set_parent(parent)
            .id();
        furnace::FurnaceNodeComponent::build(entity, world);
        smoker::SmokerNodeComponent::build(entity, world);
        blast::BlastNodeComponent::build(entity, world);
    }
}
