//! A basic [`LoadingScreen`]

use bevy::{prelude::*, ui::FocusPolicy};

mod enable;
pub use enable::*;

mod logo;
pub use logo::*;

mod progress;
pub use progress::*;

mod systemset;
pub use systemset::LoadingScreenSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<LoadingScreen>();

    systemset::build(app);
    enable::build(app);
    logo::build(app);
    progress::build(app);
}

/// A marker [`Component`] for a loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct LoadingScreen;

impl LoadingScreen {
    /// The default [`ZIndex`] for the [`LoadingScreen`].
    ///
    /// Ensures the [`LoadingScreen`] is always on top.
    pub const Z_INDEX: ZIndex = ZIndex::Global(i32::MAX / 1024);

    /// A [`System`](bevy_ecs::system::System) that spawns a
    /// [`LoadingScreen`].
    pub fn spawn(world: &mut World) {
        let entity = world.spawn_empty().id();
        Self::spawn_at(entity, Visibility::Inherited, world);
        world.entity_mut(entity).insert(Self::Z_INDEX);
    }

    /// Creates a new [`LoadingScreen`] at the given [`Entity`].
    pub fn spawn_at(entity: Entity, visibility: Visibility, world: &mut World) {
        let Some(mut entity_commands) = world.get_entity_mut(entity) else {
            error!("Failed to spawn `LoadingScreen`, entity not found!");
            return;
        };

        // Create a new NodeBundle
        let node = NodeBundle {
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Relative,

                left: Val::Percent(0.0),
                top: Val::Percent(0.0),

                height: Val::Percent(100.0),
                width: Val::Percent(100.0),

                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            focus_policy: FocusPolicy::Block,
            visibility,
            ..Default::default()
        };

        // Insert the marker and NodeBundle
        entity_commands.insert((LoadingScreen, node));

        // Spawn the logo and progress bar
        let children = entity_commands.world_scope(|world| {
            let logo_child = world.spawn_empty().id();
            LoadingScreenLogo::spawn(logo_child, world);

            let progress_child = world.spawn_empty().id();
            LoadingScreenProgressBar::spawn(progress_child, world);

            // Return the children we want to add to the parent
            [logo_child, progress_child]
        });

        // Add the children to the entity
        entity_commands.push_children(&children);
    }
}
