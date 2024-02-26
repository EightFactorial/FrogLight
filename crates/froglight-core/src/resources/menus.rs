//! `Resources` used for loading screens
use bevy::prelude::*;
use derive_more::{From, Into};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.register_type::<MainMenuEnable>().init_resource::<MainMenuEnable>();
    app.register_type::<MultiplayerMenuEnable>().init_resource::<MultiplayerMenuEnable>();
    app.register_type::<SettingsMenuEnable>().init_resource::<SettingsMenuEnable>();
}

/// A [`Resource`] that enables or disables the main menu.
///
/// By default, this is enabled at startup.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, Resource, Reflect,
)]
#[reflect(Resource)]
pub struct MainMenuEnable(pub bool);

impl MainMenuEnable {
    /// Returns `true` if the main menu is enabled.
    #[must_use]
    pub fn is_enabled(res: Res<Self>) -> bool { **res }
}

impl Default for MainMenuEnable {
    fn default() -> Self { Self(true) }
}

/// A [`Resource`] that enables or disables the multiplayer menu.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, Resource, Reflect,
)]
#[reflect(Resource)]
pub struct MultiplayerMenuEnable(pub bool);

impl MultiplayerMenuEnable {
    /// Returns `true` if the multiplayer menu is enabled.
    #[must_use]
    pub fn is_enabled(res: Res<Self>) -> bool { **res }
}

/// A [`Resource`] that enables or disables the settings menu.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, Resource, Reflect,
)]
#[reflect(Resource)]
pub struct SettingsMenuEnable(pub bool);

impl SettingsMenuEnable {
    /// Returns `true` if the settings menu is enabled.
    #[must_use]
    pub fn is_enabled(res: Res<Self>) -> bool { **res }
}
