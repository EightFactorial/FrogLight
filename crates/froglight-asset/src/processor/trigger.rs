use bevy_app::App;
use bevy_ecs::{
    event::Event,
    observer::Trigger,
    system::{Res, ResMut},
};
use bevy_state::state::NextState;

use super::{AssetProcess, ResourcePackList};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_event::<ResourceLoadTrigger>();
    app.observe(ResourceLoadTrigger::event_trigger);

    app.add_event::<ResourceResetTrigger>();
    app.observe(ResourceResetTrigger::event_trigger);
}

/// An [`Event`] that triggers the processing of
/// all [`ResourcePack`](crate::ResourcePack)s in the
/// [`ResourcePackList`](crate::ResourcePackList).
///
/// Enters the [`AssetProcess::Loading`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourceLoadTrigger;

impl ResourceLoadTrigger {
    fn event_trigger(
        _: Trigger<Self>,
        res: Res<ResourcePackList>,
        mut state: ResMut<NextState<AssetProcess>>,
    ) {
        if res.is_empty() {
            #[cfg(debug_assertions)]
            bevy_log::info!("ResourceLoadTrigger: `ResourcePackList` is empty, ignoring trigger");
        } else {
            #[cfg(debug_assertions)]
            bevy_log::info!("ResourceLoadTrigger: Entering `AssetLoadState::Loading`");
            state.set(AssetProcess::Loading);
        }
    }
}

/// An [`Event`] that triggers the reset of all
/// [`ResourcePack`](crate::ResourcePack)s in the
/// [`ResourcePackList`](crate::ResourcePackList).
///
/// Enters the [`AssetProcess::Waiting`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourceResetTrigger;

impl ResourceResetTrigger {
    fn event_trigger(_: Trigger<Self>, mut state: ResMut<NextState<AssetProcess>>) {
        #[cfg(debug_assertions)]
        bevy_log::info!("ResourceResetTrigger: Entering `AssetLoadState::Waiting`");
        state.set(AssetProcess::Waiting);
    }
}
