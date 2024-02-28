//! [`Resources`](bevy::prelude::Resource) used by all `FrogLight` crates.

use bevy_app::App;

mod loading;
pub use loading::{LoadingScreenEnable, LoadingScreenState};

mod menus;
pub use menus::{MainMenuEnable, MultiplayerMenuEnable, SettingsMenuEnable};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    loading::setup(app);
    menus::setup(app);
}
