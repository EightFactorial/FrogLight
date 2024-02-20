use std::sync::atomic::{AtomicBool, Ordering};

use bevy::prelude::*;

static BUILD_ONCE: AtomicBool = AtomicBool::new(false);

/// Configure [`SystemSet`]s
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Only run build once
    if BUILD_ONCE.load(Ordering::Relaxed) {
        return;
    }

    app.configure_sets(Startup, InterfaceStartupSet);
    app.configure_sets(Update, InterfaceUpdateSet);

    // Only run build once
    BUILD_ONCE.store(true, Ordering::Relaxed);
}

/// A [`SystemSet`] for systems that run during [`Startup`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InterfaceStartupSet;

/// A [`SystemSet`] for systems that run during [`Update`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct InterfaceUpdateSet;
