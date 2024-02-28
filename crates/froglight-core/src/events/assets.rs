use bevy_app::App;
use bevy_ecs::event::Event;
use bevy_reflect::Reflect;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ResourcePackStartLoadingEvent>()
        .add_event::<ResourcePackStartLoadingEvent>();

    app.register_type::<ResourcePackFinishedLoadingEvent>()
        .add_event::<ResourcePackFinishedLoadingEvent>();
}

/// An [`Event`] that when sent triggers resource packs to start loading.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event, Reflect)]
pub struct ResourcePackStartLoadingEvent;

/// An [`Event`] that is sent when all resource packs are finished loading.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event, Reflect)]
pub struct ResourcePackFinishedLoadingEvent;
