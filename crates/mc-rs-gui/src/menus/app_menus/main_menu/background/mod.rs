use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod cube;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundNodeComponent;

impl MenuComponent for BackgroundNodeComponent {
    fn setup(app: &mut App) { cube::BackgroundCubeComponent::setup(app); }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building BackgroundNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };

        world
            .spawn((BackgroundNodeComponent, node))
            .set_parent(parent);

        cube::BackgroundCubeComponent::build(world);
    }
}
