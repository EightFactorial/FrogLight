#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;
use derive_more::{Deref, DerefMut, From, Into};
use uuid::Uuid;

/// An entity's universally unique identifier.
///
/// This value is unique for every entity in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component, bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Component))]
pub struct EntityUuid(pub Uuid);

impl std::fmt::Display for EntityUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl AsRef<Uuid> for EntityUuid {
    fn as_ref(&self) -> &Uuid { &self.0 }
}
