use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<LoadingScreenLogoNode>(); }

/// A marker [`Component`] for the loading screen logo.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct LoadingScreenLogoNode;

impl LoadingScreenLogoNode {
    pub(super) fn build(world: &mut World, parent: Entity) {
        // Create the logo node
        let logo_node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                height: Val::Px(200.0),
                max_height: Val::Vh(50.0),
                width: Val::Px(200.0),
                max_width: Val::Vw(80.0),

                top: Val::Percent(15.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        };

        // Spawn the logo node
        world.spawn((Self, logo_node)).set_parent(parent);
    }
}
