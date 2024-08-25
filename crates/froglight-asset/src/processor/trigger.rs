use bevy_app::App;
use bevy_ecs::{
    event::Event,
    observer::Trigger,
    system::{Res, ResMut},
};
use bevy_log::{debug, warn};
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
            warn!("ResourceTrigger: No ResourcePacks, ignoring trigger");
        } else {
            debug!("ResourceTrigger: Waiting for ResourcePacks to load...");
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
        debug!("ResourceTrigger: Resetting ResourcePacks...");
        state.set(AssetProcess::Waiting);
    }
}
