use bevy_app::{App, Update};
use bevy_ecs::{
    event::{Event, EventWriter},
    schedule::{
        common_conditions::{in_state, state_changed},
        IntoSystemConfigs,
    },
};
use bevy_log::debug;

use crate::AssetLoadingState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add the `AssetsLoaded` event
    app.add_event::<AssetsLoaded>();

    // Add the `send_loaded_event` system
    app.add_systems(
        Update,
        AssetsLoaded::send_loaded_event
            .run_if(state_changed::<AssetLoadingState>)
            .run_if(in_state(AssetLoadingState::Ready))
            .in_set(AssetLoadingState::Ready),
    );
}

/// An [`Event`] that indicates that all assets have been loaded.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct AssetsLoaded;

impl AssetsLoaded {
    fn send_loaded_event(mut events: EventWriter<AssetsLoaded>) {
        debug!("Sending `AssetsLoaded` Event");
        events.send_default();
    }
}
