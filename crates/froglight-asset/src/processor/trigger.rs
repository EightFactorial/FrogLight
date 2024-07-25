use bevy_app::App;
use bevy_ecs::{
    event::Event,
    observer::Trigger,
    system::{Res, ResMut},
};
use bevy_log::warn;
use bevy_state::state::NextState;

use super::{AssetLoadState, ResourcePackList};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.add_event::<ResourceResetTrigger>();
    app.observe(ResourceResetTrigger::trigger_observer);

    app.add_event::<ResourceLoadTrigger>();
    app.observe(ResourceLoadTrigger::trigger_observer);
}

/// An [`Event`] that triggers the reset of all
/// [`ResourcePack`](crate::ResourcePack)s in the
/// [`ResourcePackList`](super::ResourcePackList).
///
/// Enters the [`AssetLoadState::Waiting`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourceResetTrigger;

impl ResourceResetTrigger {
    /// Enter the [`AssetLoadState::Waiting`] state.
    fn trigger_observer(_: Trigger<Self>, mut state: ResMut<NextState<AssetLoadState>>) {
        #[cfg(debug_assertions)]
        bevy_log::info!("ResourceResetTrigger: Entering `AssetLoadState::Waiting`");
        state.set(AssetLoadState::Waiting);
    }
}

/// An [`Event`] that triggers the processing of
/// all [`ResourcePack`](crate::ResourcePack)s in the
/// [`ResourcePackList`](super::ResourcePackList).
///
/// Enters the [`AssetLoadState::Loading`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourceLoadTrigger;

impl ResourceLoadTrigger {
    /// Enter the [`AssetLoadState::Loading`] state.
    fn trigger_observer(
        _: Trigger<Self>,
        list: Res<ResourcePackList>,
        mut state: ResMut<NextState<AssetLoadState>>,
    ) {
        if list.is_empty() {
            warn!("ResourceLoadTrigger: No ResourcePacks to load, ignoring trigger");
        } else {
            #[cfg(debug_assertions)]
            bevy_log::info!("ResourceLoadTrigger: Entering `AssetLoadState::Loading`");
            state.set(AssetLoadState::Loading);
        }
    }
}
