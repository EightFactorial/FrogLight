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

    // Only run build once
    BUILD_ONCE.store(true, Ordering::Relaxed);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InterfaceMenuUpdateSet;
