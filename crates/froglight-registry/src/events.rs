use bevy_ecs::event::Event;
use froglight_protocol::traits::Version;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    // Add the `RegistryOverrideEvent` event.
    app.add_event::<RegistryOverrideEvent>();
}

/// An [`Event`] that triggers registry values to reset.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct RegistryOverrideEvent {
    /// The [`Version`] to reset the registry values to.
    pub version_id: i32,
}

impl RegistryOverrideEvent {
    /// Creates a new [`RegistryOverrideEvent`] from a [`Version`].
    #[must_use]
    pub fn new<V: Version>() -> Self { Self { version_id: V::ID } }

    /// Creates a new [`RegistryOverrideEvent`] from a [`Version::ID`].
    #[must_use]
    pub fn from_id(version_id: i32) -> Self { Self { version_id } }

    /// Returns `true` if the [`RegistryOverrideEvent`] is for the given
    /// [`Version`].
    #[must_use]
    pub fn is_version<V: Version>(&self) -> bool { self.version_id == V::ID }
}
