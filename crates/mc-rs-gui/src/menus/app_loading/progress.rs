use bevy::prelude::*;

use crate::menus::{app_loading::FadeComponent, traits::MenuComponent};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ProgressNodeComponent;

impl MenuComponent for ProgressNodeComponent {
    fn setup(_app: &mut App) {}

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building ProgressNodeComponent");

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
            .spawn((ProgressNodeComponent, FadeComponent, node))
            .set_parent(parent);
    }
}
