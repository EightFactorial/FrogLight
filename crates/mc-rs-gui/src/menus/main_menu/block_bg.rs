use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(crate) struct BlockBackground;

impl MenuComponent for BlockBackground {
    fn setup(_app: &mut App) {}

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

        let top_bar = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Vh(10.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
            ..Default::default()
        };

        let bottom_bar = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Vh(10.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
            ..Default::default()
        };

        // Spawn MenuComponent
        world
            .spawn((BlockBackground, node))
            .with_children(|node| {
                node.spawn(top_bar);
                node.spawn(bottom_bar);
            })
            .set_parent(parent);
    }
}
