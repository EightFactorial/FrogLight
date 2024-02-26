//! `Resources` used for loading screens
use bevy::prelude::*;
use derive_more::{From, Into};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.register_type::<LoadingScreenEnable>().init_resource::<LoadingScreenEnable>();
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
