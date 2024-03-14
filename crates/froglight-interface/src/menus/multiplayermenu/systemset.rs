use bevy::prelude::*;
use froglight_core::resources::MultiplayerMenuEnable;

use crate::menus::{
    mainmenu::systemset::MainMenuUpdateSet, settingsmenu::systemset::SettingsMenuUpdateSet,
    InterfaceMenuUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        Update,
        MultiplayerMenuUpdateSet
            .run_if(MultiplayerMenuEnable::is_enabled)
            .ambiguous_with(MainMenuUpdateSet)
            .ambiguous_with(SettingsMenuUpdateSet)
            .in_set(InterfaceMenuUpdateSet),
    );
}

/// A [`SystemSet`] for multiplayer menu systems that should run during
/// [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct MultiplayerMenuUpdateSet;
