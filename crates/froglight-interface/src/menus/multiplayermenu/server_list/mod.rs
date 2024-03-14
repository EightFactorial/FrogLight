use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<MultiplayerServerListNode>(); }

/// A marker [`Component`] for the server list [`Entity`] of the multiplayer
/// menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MultiplayerServerListNode;

impl MultiplayerServerListNode {
    pub(super) fn build(world: &mut World, parent: Entity) {
        // Create the server list node
        let node = NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,

                width: Val::Px(260.0),
                height: Val::Px(200.0),

                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK.with_a(0.6)),
            ..Default::default()
        };

        // Spawn the server list node
        let _list = world
            .spawn((Self, node, Name::new("MultiplayerServerListNode")))
            .set_parent(parent)
            .id();
    }
}
