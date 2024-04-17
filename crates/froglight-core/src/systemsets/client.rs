use bevy_app::{App, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::InterfaceUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, ClientUpdateSet.after(InterfaceUpdateSet));
}

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct ClientUpdateSet;
