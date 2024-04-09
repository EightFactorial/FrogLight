use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

/// An entity's identifier.
///
/// One is assigned to an entity when it is sent to the client.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Into,
    Deref,
    DerefMut,
    FrogReadWrite,
)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
#[frog(tests = ["read_example"], bytes = [0])]
pub struct EntityId(#[frog(var)] pub u32);

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl From<u8> for EntityId {
    fn from(value: u8) -> Self { EntityId(value.into()) }
}

impl std::ops::Add<EntityId> for EntityId {
    type Output = EntityId;
    fn add(self, rhs: EntityId) -> Self::Output { EntityId(self.0.add(rhs.0)) }
}
impl std::ops::AddAssign<EntityId> for EntityId {
    fn add_assign(&mut self, rhs: EntityId) { self.0.add_assign(rhs.0) }
}
impl std::ops::Add<u8> for EntityId {
    type Output = EntityId;
    fn add(self, rhs: u8) -> Self::Output { EntityId(self.0.add(u32::from(rhs))) }
}
impl std::ops::AddAssign<u8> for EntityId {
    fn add_assign(&mut self, rhs: u8) { self.0.add_assign(u32::from(rhs)) }
}

impl std::ops::Sub<EntityId> for EntityId {
    type Output = EntityId;
    fn sub(self, rhs: EntityId) -> Self::Output { EntityId(self.0.sub(rhs.0)) }
}
impl std::ops::SubAssign<EntityId> for EntityId {
    fn sub_assign(&mut self, rhs: EntityId) { self.0.sub_assign(rhs.0) }
}
impl std::ops::Sub<u8> for EntityId {
    type Output = EntityId;
    fn sub(self, rhs: u8) -> Self::Output { EntityId(self.0.sub(u32::from(rhs))) }
}
impl std::ops::SubAssign<u8> for EntityId {
    fn sub_assign(&mut self, rhs: u8) { self.0.sub_assign(u32::from(rhs)) }
}

impl AsRef<u32> for EntityId {
    fn as_ref(&self) -> &u32 { &self.0 }
}
