use bevy_app::App;
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::Reflect;
use derive_more::{Deref, DerefMut};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ShowingSplash>()
        .init_resource::<ShowingMainMenu>()
        .init_resource::<ShowingSettings>()
        .init_resource::<ShowingGame>()
        .init_resource::<IsPaused>();

    app.register_type::<ShowingSplash>()
        .register_type::<ShowingMainMenu>()
        .register_type::<ShowingSettings>()
        .register_type::<ShowingGame>()
        .register_type::<IsPaused>();
}

/// A [`Resource`] that indicates whether the splash screen is currently being
/// shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Resource)]
pub struct ShowingSplash(pub bool);

impl Default for ShowingSplash {
    fn default() -> Self { ShowingSplash(true) }
}

/// A [`Resource`] that indicates whether the main menu is currently being
/// shown.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct ShowingMainMenu(pub bool);

/// A [`Resource`] that indicates whether the settings are currently being
/// shown.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct ShowingSettings(pub bool);

/// A [`Resource`] that indicates whether the game is currently being shown.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct ShowingGame(pub bool);

/// A [`Resource`] that indicates whether the game is currently paused.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct IsPaused(pub bool);
