use bevy_app::App;
use bevy_ecs::event::Event;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.add_event::<RegistryUpdateEvent>(); }

/// An [`Event`] that triggers registry values to reset
/// to the default values of a specific `Version`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct RegistryUpdateEvent {
    /// The `Version::ID` of the `Version`
    /// to reset the registry values to.
    pub version_id: i32,
}
