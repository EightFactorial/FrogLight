use bevy::prelude::*;

use crate::resources::servers::ServerItem;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct PlayersNodeComponent;

impl PlayersNodeComponent {
    pub(super) fn setup(_app: &mut App) {}

    pub(super) fn build(_server: &ServerItem, parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building PlayersNodeComponent");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };

        world.spawn((PlayersNodeComponent, node)).set_parent(parent);
    }
}
