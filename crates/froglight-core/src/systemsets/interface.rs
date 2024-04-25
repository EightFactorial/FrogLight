use bevy_app::{App, PostUpdate, PreUpdate, Startup, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{ClientPostUpdateSet, ClientPreUpdateSet};

/// All `Interface` [`SystemSets`](SystemSet) run after `Asset`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, InterfaceStartupSet)
        .configure_sets(PreUpdate, InterfacePreUpdateSet.after(ClientPreUpdateSet))
        .configure_sets(Update, InterfaceUpdateSet)
        .configure_sets(PostUpdate, InterfacePostUpdateSet.after(ClientPostUpdateSet));
}

/// A [`SystemSet`] that runs during the [`Startup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfaceStartupSet;

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfacePreUpdateSet;

/// A [`SystemSet`] that runs during the [`Update`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfaceUpdateSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfacePostUpdateSet;
