use std::num::TryFromIntError;

use bevy_ecs::prelude::Component;
use derive_more::{Deref, DerefMut};

use crate::buffer::{Decode, Encode, VarDecode, VarEncode};

/// A Minecraft entity ID.
///
/// Very different from a Bevy [Entity](bevy_ecs::entity::Entity).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut)]
pub struct EntityId(pub u32);

impl From<EntityId> for u32 {
    fn from(id: EntityId) -> Self { id.0 }
}

impl From<u32> for EntityId {
    fn from(id: u32) -> Self { Self(id) }
}

impl From<u16> for EntityId {
    fn from(id: u16) -> Self { Self(id.into()) }
}

impl From<u8> for EntityId {
    fn from(id: u8) -> Self { Self(id.into()) }
}

impl TryFrom<i32> for EntityId {
    type Error = TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> { Ok(Self(u32::try_from(value)?)) }
}

impl TryFrom<i16> for EntityId {
    type Error = TryFromIntError;

    fn try_from(value: i16) -> Result<Self, Self::Error> { Ok(Self(u32::try_from(value)?)) }
}

impl TryFrom<i8> for EntityId {
    type Error = TryFromIntError;

    fn try_from(value: i8) -> Result<Self, Self::Error> { Ok(Self(u32::try_from(value)?)) }
}

impl Encode for EntityId {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.0.var_encode(buf)
    }
}

impl Decode for EntityId {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(EntityId(u32::var_decode(buf)?))
    }
}
