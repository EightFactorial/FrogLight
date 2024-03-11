use bevy::prelude::*;

use crate::menus::{
    mainmenu::systemset::MainMenuUpdateSet, multiplayermenu::systemset::MultiplayerMenuUpdateSet,
    InterfaceMenuUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        Update,
        SettingsMenuUpdateSet
            .ambiguous_with(MainMenuUpdateSet)
            .ambiguous_with(MultiplayerMenuUpdateSet)
            .in_set(InterfaceMenuUpdateSet),
    );
}

/// A [`SystemSet`] for settings menu systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct SettingsMenuUpdateSet;
