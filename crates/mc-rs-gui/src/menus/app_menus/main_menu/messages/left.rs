use bevy::{prelude::*, text::BreakLineOn};

use crate::{menus::traits::MenuComponent, resources::font::DefaultTextStyle};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct LeftNodeComponent;

impl MenuComponent for LeftNodeComponent {
    fn setup(_app: &mut App) {}

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building LeftNodeComponent");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::Start,
                ..Default::default()
            },
            ..Default::default()
        };

        let node = world
            .spawn((LeftNodeComponent, node))
            .set_parent(parent)
            .id();

        let value: String;
        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            value = format!("MC-RS v{} - Nightly", env!("CARGO_PKG_VERSION"),);
        }
        #[cfg(not(any(debug_assertions, feature = "debug")))]
        {
            value = format!("MC-RS v{}", env!("CARGO_PKG_VERSION"));
        }

        let style: TextStyle = world.resource::<DefaultTextStyle>().clone().into();

        let mut style_but_red = style.clone();
        style_but_red.color = Color::CRIMSON;

        world
            .spawn(TextBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(2.0),
                        bottom: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::from("Extreme Alpha!\n"),
                            style: style_but_red,
                        },
                        TextSection { value, style },
                    ],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..Default::default()
            })
            .set_parent(node);
    }
}
