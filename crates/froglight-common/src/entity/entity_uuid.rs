#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::ReflectDefault;
use derive_more::{Deref, DerefMut, From, Into};
use uuid::Uuid;

/// An entity's universally unique identifier.
///
/// This value is unique for every entity in the world.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into, Deref, DerefMut,
)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component, bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Component, Default))]
pub struct EntityUuid(pub Uuid);

impl EntityUuid {
    /// Creates a new [`EntityUuid`] with the given value.
    ///
    /// Be careful, only ***very*** rarely should you need to use this.
    #[must_use]
    pub const fn new(value: Uuid) -> Self { EntityUuid(value) }
}

impl std::fmt::Display for EntityUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl AsRef<Uuid> for EntityUuid {
    fn as_ref(&self) -> &Uuid { &self.0 }
}
impl AsRef<[u8; 16]> for EntityUuid {
    fn as_ref(&self) -> &[u8; 16] { self.0.as_bytes() }
}
impl AsRef<[u8]> for EntityUuid {
    fn as_ref(&self) -> &[u8] { self.0.as_bytes() }
}

impl From<u128> for EntityUuid {
    fn from(value: u128) -> Self { EntityUuid(Uuid::from_u128(value)) }
}
impl From<EntityUuid> for u128 {
    fn from(value: EntityUuid) -> Self { value.0.as_u128() }
}

impl From<[u8; 16]> for EntityUuid {
    fn from(value: [u8; 16]) -> Self { EntityUuid(Uuid::from_bytes(value)) }
}
impl From<EntityUuid> for [u8; 16] {
    fn from(value: EntityUuid) -> Self { value.0.into_bytes() }
}
