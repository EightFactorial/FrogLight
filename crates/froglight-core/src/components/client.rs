use bevy_app::App;
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ClientPlayer>().register_type::<ClientCamera>();
}

/// A [`Component`] that marks an entity as the client's player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct ClientPlayer;

/// A [`Component`] that marks an entity as the client's camera.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct ClientCamera;
