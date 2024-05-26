use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.configure_sets(Startup, ClientStartupDiagnosticsSet); }

/// A [`SystemSet`] containing systems for client diagnostics run on startup.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClientStartupDiagnosticsSet;
