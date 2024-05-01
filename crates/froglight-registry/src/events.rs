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

impl RegistryUpdateEvent {
    /// Creates a new [`RegistryUpdateEvent`] with the given version id
    #[must_use]
    pub fn new(version_id: i32) -> Self { Self { version_id } }

    /// Returns `true` if the version id is equal to `N`.
    #[must_use]
    pub fn is_equal<const ID: i32>(&self) -> bool { self.version_id == ID }
}
