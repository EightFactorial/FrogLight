use bevy::prelude::*;
use froglight_core::resources::MainMenuEnable;

use crate::menus::InterfaceMenuUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(
        Update,
        MainMenuUpdateSet.run_if(MainMenuEnable::is_enabled).in_set(InterfaceMenuUpdateSet),
    );
}

/// A [`SystemSet`] for main menu systems that should run during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct MainMenuUpdateSet;
