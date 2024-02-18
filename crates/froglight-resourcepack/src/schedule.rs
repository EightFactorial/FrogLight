//! Schedules for loading resource packs.

use bevy::{ecs::schedule::BoxedCondition, prelude::*};
use froglight_core::{
    events::{ResourcePackFinishedLoadingEvent, ResourcePackStartLoadingEvent},
    systemsets::ResourcePackUpdateSet,
};

use crate::ResourcePackPlugin;

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
            .in_set(ResourcePackUpdateSet),
    );

    app.add_systems(
        Update,
        ResourcePackState::enter_loading_state
            .run_if(on_event::<ResourcePackStartLoadingEvent>())
            .in_set(ResourcePackState::Waiting),
    );
}

#[doc(hidden)]
pub(super) fn finish(plugin: &ResourcePackPlugin, app: &mut App) {
    // Create the system that enters the ready state.
    let mut system = ResourcePackState::enter_ready_state.into_configs();

    // Take the conditions from the plugin.
    let conditions: Vec<BoxedCondition> = std::mem::take(plugin.conditions.lock().as_mut());
    let condition_count = conditions.len();

    // Add the conditions to the system.
    for cond in conditions {
        system.run_if_dyn(cond);
    }

    // Add the system to the `Processing` set.
    debug!(
        "Adding ResourcePackState::enter_ready_state system with `{condition_count}` conditions"
    );
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
