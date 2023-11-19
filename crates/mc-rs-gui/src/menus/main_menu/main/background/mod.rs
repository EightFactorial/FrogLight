use bevy::{prelude::*, ui::FocusPolicy};

use crate::menus::traits::MenuComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubemapBackground;

impl MenuComponent for CubemapBackground {
    fn setup(_app: &mut App) {
        // TODO: Add systems
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building CubemapBackground");

        // Create node
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            focus_policy: FocusPolicy::Pass,
            ..Default::default()
        };

        // Spawn CubemapBackground
        world.spawn((CubemapBackground, node)).set_parent(parent);
    }
}
