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

impl AsRef<u32> for EntityId {
    fn as_ref(&self) -> &u32 { &self.0 }
}
