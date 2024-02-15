use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ResourcePackStartLoadingEvent>()
        .register_type::<ResourcePackEndLoadingEvent>()
        .register_type::<ResourcePackFinishLoadingEvent>();

    app.add_event::<ResourcePackStartLoadingEvent>()
        .add_event::<ResourcePackEndLoadingEvent>()
        .add_event::<ResourcePackFinishLoadingEvent>();
}

/// An [`Event`]that is fired when resource packs start loading.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event, Reflect)]
pub struct ResourcePackStartLoadingEvent;

/// An [`Event`] that is fired when all resource packs have finished loading,
/// but before they have finished processing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event, Reflect)]
pub struct ResourcePackEndLoadingEvent;

/// An [`Event`] that is fired when all resource packs have finished processing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event, Reflect)]
pub struct ResourcePackFinishLoadingEvent;
