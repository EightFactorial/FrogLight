//! Menu plugins

use std::sync::atomic::{AtomicBool, Ordering};

use bevy::prelude::*;
use froglight_core::systemsets::InterfaceUpdateSet;

pub mod loadingscreen;
pub use loadingscreen::plugin::InterfaceLoadingScreenPlugin;

pub mod mainmenu;
pub use mainmenu::plugin::InterfaceMainMenuPlugin;

pub mod multiplayermenu;
pub use multiplayermenu::plugin::InterfaceMultiplayerMenuPlugin;

pub mod panorama;
pub use panorama::plugin::InterfacePanoramaPlugin;

pub mod settingsmenu;
pub use settingsmenu::plugin::InterfaceSettingsMenuPlugin;

static BUILD_ONCE: AtomicBool = AtomicBool::new(false);

#[doc(hidden)]
fn build(app: &mut App) {
    // Only run build once
    if BUILD_ONCE.load(Ordering::Relaxed) {
        return;
    }

    app.configure_sets(Update, InterfaceMenuUpdateSet.in_set(InterfaceUpdateSet));

    app.register_type::<InterfaceMenuState>()
        .init_state::<InterfaceMenuState>()
        .register_type::<State<InterfaceMenuState>>()
        .register_type_data::<State<InterfaceMenuState>, ReflectResource>();

    // Only run build once
    BUILD_ONCE.store(true, Ordering::Relaxed);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InterfaceMenuUpdateSet;

/// The state of the interface menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum InterfaceMenuState {
    /// The Main Menu
    #[default]
    MainMenu,
    /// The Multiplayer Menu
    MultiplayerMenu,
    /// The Settings Menu
    SettingsMenu,
}
