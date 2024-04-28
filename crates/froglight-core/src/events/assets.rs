use bevy_app::App;
use bevy_ecs::event::Event;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_event::<AssetsStartLoading>()
        .add_event::<AssetsStartProcessing>()
        .add_event::<AssetsFinishLoading>();
}

/// An [`Event`] that is triggered when assets start loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct AssetsStartLoading;

/// An [`Event`] that is triggered when assets start processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct AssetsStartProcessing;

/// An [`Event`] that is triggered when assets finish loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct AssetsFinishLoading;
