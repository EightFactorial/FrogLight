use bevy_app::App;
use bevy_asset::embedded_asset;
use bevy_ecs::{component::Component, entity::Entity, world::World};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    embedded_asset!(app, "assets/froglight_logo.png");
}

/// A marker [`Component`] for the [`LoadingScreen`](super::LoadingScreen) logo.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct LoadingScreenLogo;

impl LoadingScreenLogo {
    /// Spawns a [`LoadingScreenLogo`] at the given [`Entity`].
    pub fn spawn(_entity: Entity, _world: &mut World) {}
}
