use bevy_app::{App, PostUpdate, Startup};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};

/// All `Interface` [`SystemSets`](SystemSet) run after `Asset`
/// [`SystemSets`](SystemSet).
#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Startup, InterfaceStartupSet.after(super::AssetStartupSet))
        .configure_sets(PostUpdate, InterfacePostUpdateSet);
}

/// A [`SystemSet`] that runs during the [`Startup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfaceStartupSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InterfacePostUpdateSet;
