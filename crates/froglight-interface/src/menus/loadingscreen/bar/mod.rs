use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<ProgressBarNode>(); }

/// A marker [`Component`] for the progress bar node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct ProgressBarNode;

impl ProgressBarNode {
    pub(super) fn build(world: &mut World, parent: Entity) {
        // Create the progress bar node
        let bar_node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                height: Val::Px(20.0),
                width: Val::Percent(80.0),

                bottom: Val::Percent(10.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        };

        // Spawn the progress bar node
        world.spawn((Self, bar_node)).set_parent(parent);
    }
}
