//! `Resources` used for loading screens

use bevy_app::App;
use bevy_ecs::{
    reflect::ReflectResource,
    system::{Res, Resource},
};
use bevy_reflect::Reflect;
use derive_more::{Deref, DerefMut, From, Into};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.register_type::<LoadingScreenEnable>().init_resource::<LoadingScreenEnable>();
    app.register_type::<LoadingScreenState>().init_resource::<LoadingScreenState>();
}

/// A [`Resource`] that enables or disables the loading screen.
///
/// By default, this is enabled at startup.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, Resource, Reflect,
)]
#[reflect(Resource)]
pub struct LoadingScreenEnable(pub bool);

impl LoadingScreenEnable {
    /// Returns `true` if the loading screen is enabled.
    #[must_use]
    pub fn is_enabled(res: Res<Self>) -> bool { **res }
}

impl Default for LoadingScreenEnable {
    fn default() -> Self { Self(true) }
}

/// A [`Resource`] that stores the current state of the loading screen.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, Resource, Reflect,
)]
#[reflect(Resource)]
pub struct LoadingScreenState(pub bool);

impl LoadingScreenState {
    /// Returns `true` if the loading screen is currently visible.
    #[must_use]
    pub fn is_visible(res: Res<Self>) -> bool { **res }

    /// Returns `true` if the loading screen is currently hidden.
    #[must_use]
    pub fn is_hidden(res: Res<Self>) -> bool { !**res }
}

impl Default for LoadingScreenState {
    fn default() -> Self { Self(true) }
}
