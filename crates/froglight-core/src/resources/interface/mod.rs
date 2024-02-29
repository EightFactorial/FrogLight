use bevy_app::App;

mod loading;
pub use loading::{LoadingScreenEnable, LoadingScreenState};

mod menus;
pub use menus::{MainMenuEnable, MultiplayerMenuEnable, SettingsMenuEnable};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    loading::setup(app);
    menus::setup(app);
}
