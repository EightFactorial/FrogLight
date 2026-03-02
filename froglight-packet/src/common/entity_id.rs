//! TODO

use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "facet")]
use facet::{Facet, Partial, Peek};
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, bytes_to_variable, error::DeserializeValueError},
    replace_with::replace_with_or_abort,
    serialize::{buffer::SerializeWriter, error::SerializeIterError, variable_to_bytes},
};
#[cfg(feature = "facet")]
use froglight_common::entity::EntityId;

/// An [`EntityId`] that uses variable-length encoding.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet), facet(transparent))]
#[cfg_attr(feature = "facet", facet(mc::serialize = VarEntityId::SERIALIZE))]
#[cfg_attr(feature = "facet", facet(mc::deserialize = VarEntityId::DESERIALIZE))]
pub struct VarEntityId(pub EntityId);

#[cfg(feature = "facet")]
impl VarEntityId {
    const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    #[expect(clippy::cast_possible_truncation, reason = "Desired behavior")]
    fn facet_deserialize<'facet, const BORROW: bool>(
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let (len, value) = bytes_to_variable(cursor.as_slice())?;
        cursor.consume(len)?;
        let value = Self(EntityId::new(value as i32));
        replace_with_or_abort(partial, |partial| partial.set(value).unwrap());
        Ok(())
    }

    #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
    fn facet_serialize<'mem, 'facet>(
        peek: Peek<'mem, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        let value = peek.get::<Self>()?;
        let mut buffer = [0; _];
        let len = variable_to_bytes(value.0.0 as u128, &mut buffer);
        if writer.write_data(&buffer[..len]) { Ok(()) } else { Err(SerializeIterError::new()) }
    }
}

impl AsRef<EntityId> for VarEntityId {
    fn as_ref(&self) -> &EntityId { &self.0 }
}
impl AsMut<EntityId> for VarEntityId {
    fn as_mut(&mut self) -> &mut EntityId { &mut self.0 }
}

impl Borrow<EntityId> for VarEntityId {
    fn borrow(&self) -> &EntityId { &self.0 }
}
impl BorrowMut<EntityId> for VarEntityId {
    fn borrow_mut(&mut self) -> &mut EntityId { &mut self.0 }
}

impl Deref for VarEntityId {
    type Target = EntityId;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for VarEntityId {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl PartialEq<EntityId> for VarEntityId {
    fn eq(&self, other: &EntityId) -> bool { self.0 == *other }
}
impl PartialEq<VarEntityId> for EntityId {
    fn eq(&self, other: &VarEntityId) -> bool { *self == other.0 }
}

impl PartialOrd<EntityId> for VarEntityId {
    fn partial_cmp(&self, other: &EntityId) -> Option<Ordering> { Some(self.0.cmp(other)) }
}
impl PartialOrd<VarEntityId> for EntityId {
    fn partial_cmp(&self, other: &VarEntityId) -> Option<Ordering> { Some(self.cmp(&other.0)) }
}
