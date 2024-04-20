use bevy_app::{App, PostUpdate, PreUpdate, Startup, Update};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

use super::{AssetStartupSet, ClientUpdateSet, NetworkPreUpdateSet};

/// All `Interface` [`SystemSets`](SystemSet) run after `Asset`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, InterfaceStartupSet.after(AssetStartupSet))
        .configure_sets(PreUpdate, InterfacePreUpdateSet.after(NetworkPreUpdateSet))
        .configure_sets(Update, InterfaceUpdateSet.after(ClientUpdateSet))
        .configure_sets(PostUpdate, InterfacePostUpdateSet);
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
