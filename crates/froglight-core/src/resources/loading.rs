//! `Resources` used for loading screens
use bevy::prelude::*;
use derive_more::{From, Into};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) { app.init_resource::<LoadingScreenEnable>(); }

/// A `Resource` that enables or disables the loading screen.
///
/// By default, this is enabled at startup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, Resource)]
pub struct LoadingScreenEnable(pub bool);

impl Default for LoadingScreenEnable {
    fn default() -> Self { Self(true) }
}
