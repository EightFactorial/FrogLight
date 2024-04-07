use derive_more::{Deref, DerefMut, From, Into};
use uuid::Uuid;

/// An entity's universally unique identifier.
///
/// This value is unique for every entity in the world.
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
    /* FrogReadWrite, */
)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
// #[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 0,
// 0, 0, 0, 0])]
pub struct EntityUuid(Uuid);

impl std::fmt::Display for EntityUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl AsRef<Uuid> for EntityUuid {
    fn as_ref(&self) -> &Uuid { &self.0 }
}
