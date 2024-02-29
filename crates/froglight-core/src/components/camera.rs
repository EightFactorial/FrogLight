use bevy_app::App;
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) { app.register_type::<PlayerCamera>(); }

/// A marker [`Component`] for the player's camera.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct PlayerCamera;
