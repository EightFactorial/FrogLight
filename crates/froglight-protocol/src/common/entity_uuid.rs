use bevy_reflect::Reflect;
use derive_more::{Deref, DerefMut, From, Into};
use uuid::Uuid;

/// An entity's universally unique identifier.
///
/// This value is unique for every entity in the world.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into, Deref, DerefMut, Reflect,
)]
// TODO: #[frog(tests = ["read_example"], bytes = [])]
pub struct EntityUuid(Uuid);

impl AsRef<Uuid> for EntityUuid {
    fn as_ref(&self) -> &Uuid { &self.0 }
}
