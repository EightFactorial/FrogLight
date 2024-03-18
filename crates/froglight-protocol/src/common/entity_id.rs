use std::fmt::Display;

use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

/// An entity's unique identifier.
///
/// One is assigned to an entity when it is sent to the client.
///
/// ---
///
/// This is different than bevy's `Entity` type.
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
#[frog(tests = ["read_example"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityId(#[frog(var)] u32);

impl AsRef<u32> for EntityId {
    fn as_ref(&self) -> &u32 { &self.0 }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for EntityId {
                fn from(id: $t) -> Self {
                    Self(u32::from(id))
                }
            }
        )*
    };
    (try $($t:ty),*) => {
        $(
            impl TryFrom<$t> for EntityId {
                type Error = std::num::TryFromIntError;
                fn try_from(id: $t) -> Result<Self, Self::Error> {
                    Ok(Self(u32::try_from(id)?))
                }
            }
        )*
    };
}
impl_from!(u8, u16);
impl_from!(try u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl std::ops::Add for EntityId {
    type Output = EntityId;
    fn add(self, rhs: EntityId) -> Self::Output { EntityId(self.0 + rhs.0) }
}

impl std::ops::AddAssign for EntityId {
    fn add_assign(&mut self, rhs: EntityId) { self.0 += rhs.0; }
}

impl std::ops::Sub for EntityId {
    type Output = EntityId;
    fn sub(self, rhs: EntityId) -> Self::Output { EntityId(self.0 - rhs.0) }
}

impl std::ops::SubAssign for EntityId {
    fn sub_assign(&mut self, rhs: EntityId) { self.0 -= rhs.0; }
}
