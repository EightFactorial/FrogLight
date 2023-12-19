use bevy::prelude::*;

pub(super) fn setup(app: &mut App) {
    app.add_state::<MenuComponentState>();

    app.configure_sets(
        Update,
        (
            MenuComponentMenusSet.run_if(in_state(MenuComponentState::Menus)),
            MenuComponentInGameSet.run_if(in_state(MenuComponentState::InGame)),
        )
            .chain(),
    );

    #[cfg(any(debug_assertions, feature = "debug"))]
    app.add_systems(
        OnEnter(MenuComponentState::Menus),
        MenuComponentMenusSet::enter.in_set(MenuComponentMenusSet),
    );

    #[cfg(any(debug_assertions, feature = "debug"))]
    app.add_systems(
        OnEnter(MenuComponentState::InGame),
        MenuComponentInGameSet::enter.in_set(MenuComponentInGameSet),
    );
}

/// An enum of the different menus that can be shown.
///
/// This is used to determine which [`SystemSet`] to run.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum MenuComponentState {
    #[default]
    Menus,
    InGame,
}

/// A [`SystemSet`] that runs when the [`MenuComponentState`] is [`MenuComponentState::Menus`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuComponentMenusSet;

#[cfg(any(debug_assertions, feature = "debug"))]
impl MenuComponentMenusSet {
    fn enter() {
        debug!("Entering MenuComponentMenus");
    }
}

/// A [`SystemSet`] that runs when the [`MenuComponentState`] is [`MenuComponentState::InGame`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MenuComponentInGameSet;

#[cfg(any(debug_assertions, feature = "debug"))]
impl MenuComponentInGameSet {
    fn enter() {
        debug!("Entering MenuComponentInGame");
    }
}
