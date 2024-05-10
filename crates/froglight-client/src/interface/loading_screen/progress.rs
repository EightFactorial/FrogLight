use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<LoadingScreenProgressBar>().register_type::<LoadingScreenProgress>();
}

/// A marker [`Component`] for the [`LoadingScreen`](super::LoadingScreen)
/// progress bar.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct LoadingScreenProgressBar;

impl LoadingScreenProgressBar {
    /// Spawns a [`LoadingScreenProgressBar`] at the given [`Entity`].
    pub fn spawn(_entity: Entity, _world: &mut World) {}
}

/// A [`Component`] that represents the progress of a
/// [`LoadingScreen`](super::LoadingScreen).
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Component, Reflect)]
#[reflect(Component)]
pub struct LoadingScreenProgress(pub f32);

impl LoadingScreenProgress {
    /// Creates a new [`LoadingScreenProgress`] with the given progress.
    #[must_use]
    pub const fn new(progress: f32) -> Self { Self(progress) }
}
