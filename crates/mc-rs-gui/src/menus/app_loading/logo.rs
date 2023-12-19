use bevy::prelude::*;

use crate::menus::{app_loading::FadeComponent, traits::MenuComponent};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct LogoNodeComponent;

impl MenuComponent for LogoNodeComponent {
    fn setup(_app: &mut App) {}

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building LogoNodeComponent");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };

        world
            .spawn((LogoNodeComponent, FadeComponent, node))
            .with_children(|node| {
                node.spawn((
                    FadeComponent,
                    NodeBundle {
                        style: Style {
                            top: Val::Percent(33.0),
                            height: Val::Vh(17.0),
                            width: Val::Vh(17.0),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    },
                ));
            })
            .set_parent(parent);
    }
}
