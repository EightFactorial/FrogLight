use bevy::prelude::*;

use crate::interface::SCALE_WIDTH_F32;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<LoadingScreenProgressBar>().register_type::<LoadingScreenProgress>();
}

/// A marker [`Component`] for the [`LoadingScreen`](super::LoadingScreen)
/// progress bar.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LoadingScreenProgressBar;

impl LoadingScreenProgressBar {
    /// Spawns a [`LoadingScreenProgressBar`], returning the [`Entity`].
    pub fn spawn(world: &mut World) -> Entity {
        // Spawn a new progress bar
        let entity = world.spawn_empty().id();
        Self::spawn_at(entity, world);

        // Return the `Entity`
        entity
    }

    /// Spawns a [`LoadingScreenProgressBar`] at the given [`Entity`].
    pub fn spawn_at(entity: Entity, world: &mut World) {
        debug!("Entity {entity:?} - Spawning a new `LoadingScreenProgressBar`");
        let Some(mut entity_commands) = world.get_entity_mut(entity) else {
            error!("Failed to spawn `LoadingScreenProgressBar`, Entity not found!");
            return;
        };

        // Create a new NodeBundle
        #[allow(clippy::cast_precision_loss)]
        let node = NodeBundle {
            style: Style {
                margin: UiRect::top(Val::Px(40.0)),
                width: Val::Px(SCALE_WIDTH_F32 - 8.0),
                height: Val::Px(10.0),
                ..Default::default()
            },
            background_color: Color::WHITE.into(),
            ..Default::default()
        };

        // Insert the marker and bundle
        entity_commands.insert((
            LoadingScreenProgressBar,
            Name::new("LoadingScreenProgressBar"),
            node,
        ));
    }
}

/// A [`Component`] that represents the progress of a
/// [`LoadingScreen`](super::LoadingScreen).
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LoadingScreenProgress(pub f32);

impl LoadingScreenProgress {
    /// Creates a new [`LoadingScreenProgress`] with the given progress.
    #[must_use]
    pub const fn new(progress: f32) -> Self { Self(progress) }
}
