use bevy::prelude::*;

use crate::menus::{app_loading::FadeComponent, traits::MenuComponent};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundNodeComponent;

impl MenuComponent for BackgroundNodeComponent {
    fn setup(_app: &mut App) {}

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
            background_color: Color::BLACK.into(),
            ..Default::default()
        };

        world
            .spawn((BackgroundNodeComponent, FadeComponent, node))
            .set_parent(parent);
    }
}
