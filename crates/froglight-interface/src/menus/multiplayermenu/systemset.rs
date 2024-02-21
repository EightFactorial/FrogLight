use bevy::prelude::*;

use crate::menus::InterfaceMenuUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, MultiplayerMenuUpdateSet.in_set(InterfaceMenuUpdateSet));
}

/// A [`SystemSet`] for multiplayer menu systems that should run during
/// [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct MultiplayerMenuUpdateSet;
