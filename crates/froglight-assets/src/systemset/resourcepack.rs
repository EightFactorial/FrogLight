//! Schedules for loading resource packs.

use bevy_app::{App, Update};
use bevy_ecs::{
    event::EventWriter,
    schedule::{
        common_conditions::{in_state, on_event},
        BoxedCondition, IntoSystemConfigs, IntoSystemSetConfigs, NextState, States, SystemSet,
    },
    system::ResMut,
};
use bevy_log::debug;
use bevy_reflect::Reflect;
use froglight_core::{
    events::{ResourcePackFinishedLoadingEvent, ResourcePackStartLoadingEvent},
    systemsets::AssetUpdateSet,
};
use parking_lot::Mutex;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ResourcePackState>().init_state::<ResourcePackState>();

    app.configure_sets(
        Update,
        (
            ResourcePackState::Waiting.run_if(in_state(ResourcePackState::Waiting)),
            ResourcePackState::Loading.run_if(in_state(ResourcePackState::Loading)),
            ResourcePackState::Processing.run_if(in_state(ResourcePackState::Processing)),
            ResourcePackState::Ready.run_if(in_state(ResourcePackState::Ready)),
        )
            .chain()
            .in_set(AssetUpdateSet),
    );

    app.add_systems(
        Update,
        ResourcePackState::enter_loading_state
            .run_if(on_event::<ResourcePackStartLoadingEvent>())
            .in_set(ResourcePackState::Waiting),
    );
}

#[doc(hidden)]
pub(super) fn finish(conditions: &Mutex<Vec<BoxedCondition>>, app: &mut App) {
    let conditions: Vec<BoxedCondition> = std::mem::take(&mut conditions.lock());
    debug!("Adding {} ResourcePackState::Processing conditions", conditions.len());

    // Add the conditions to the system.
    let mut system = ResourcePackState::enter_ready_state.into_configs();
    for cond in conditions {
        system.run_if_dyn(cond);
    }

    // Add the system
    app.add_systems(Update, system.in_set(ResourcePackState::Processing));
}

/// The possible states of the resource pack system.
///
/// This is used to manage the loading and processing of resource packs.
///
/// By default, the state is [`ResourcePackState::Waiting`].
///
/// ---
///
/// After a [`ResourcePackStartLoadingEvent`] is sent, the state transitions to
/// [`ResourcePackState::Loading`].
///
/// Once all [`ResourcePack`]s are loaded, the state transitions to
/// [`ResourcePackState::Processing`].
///
/// Finally, once all assets are processed, the state transitions to
/// [`ResourcePackState::Ready`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, SystemSet, States)]
pub enum ResourcePackState {
    /// The default state.
    ///
    /// Waiting for a [`ResourcePackStartLoadingEvent`] to be sent.
    #[default]
    Waiting,
    /// Loading resource packs.
    Loading,
    /// Processing assets.
    Processing,
    /// All assets are processed and ready to use.
    Ready,
}

impl ResourcePackState {
    fn enter_loading_state(mut state: ResMut<NextState<ResourcePackState>>) {
        debug!("Entering ResourcePackState::Loading");
        state.set(ResourcePackState::Loading);
    }

    fn enter_ready_state(
        mut state: ResMut<NextState<ResourcePackState>>,
        mut events: EventWriter<ResourcePackFinishedLoadingEvent>,
    ) {
        debug!("Sending ResourcePackFinishedLoadingEvent");
        events.send(ResourcePackFinishedLoadingEvent);

        debug!("Entering ResourcePackState::Ready");
        state.set(ResourcePackState::Ready);
    }
}
