use bevy::prelude::*;

use crate::traits::interface::InterfaceComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct MultiplayerBackground;

impl InterfaceComponent for MultiplayerBackground {
    fn setup(_app: &mut App) {}

    fn build(multiplayer: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MultiplayerBackground");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        };

        let todo = TextBundle {
            text: Text::from_section(
                "UI is hard :(",
                TextStyle {
                    font_size: 32.0,
                    ..Default::default()
                },
            ),
            ..Default::default()
        };

        let background = world
            .spawn((MultiplayerBackground, node))
            .with_children(|node| {
                node.spawn(todo);
            })
            .id();

        world.entity_mut(multiplayer).add_child(background);
    }
}

impl MultiplayerBackground {}
