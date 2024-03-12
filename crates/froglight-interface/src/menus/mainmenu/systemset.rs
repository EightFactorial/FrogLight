use bevy::prelude::*;
use froglight_core::resources::MainMenuEnable;

use crate::menus::{
    multiplayermenu::systemset::MultiplayerMenuUpdateSet,
    settingsmenu::systemset::SettingsMenuUpdateSet, InterfaceMenuState, InterfaceMenuUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        Update,
        MainMenuUpdateSet
            .run_if(MainMenuEnable::is_enabled)
            .run_if(in_state(InterfaceMenuState::MainMenu))
            .ambiguous_with(MultiplayerMenuUpdateSet)
            .ambiguous_with(SettingsMenuUpdateSet)
            .in_set(InterfaceMenuUpdateSet),
    );
}

/// A [`SystemSet`] for main menu systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct MainMenuUpdateSet;
