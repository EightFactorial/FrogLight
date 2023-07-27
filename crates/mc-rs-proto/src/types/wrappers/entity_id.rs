use bevy_ecs::prelude::Component;
use derive_more::{Deref, DerefMut};

use crate::buffer::{Decode, Encode, VarDecode, VarEncode};

/// A Minecraft entity ID.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut)]
pub struct EntityId(pub u32);

impl From<u32> for EntityId {
    fn from(id: u32) -> Self { Self(id) }
}

impl From<EntityId> for u32 {
    fn from(id: EntityId) -> Self { id.0 }
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
