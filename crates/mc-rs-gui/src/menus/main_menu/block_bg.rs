use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(crate) struct BlockBackground;

impl MenuComponent for BlockBackground {
    fn setup(_app: &mut App) {
        // TODO: Add systems
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building BlockBackground");

        // Create node
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };

        // Spawn MenuComponent
        world.spawn((BlockBackground, node)).set_parent(parent);
    }
}
