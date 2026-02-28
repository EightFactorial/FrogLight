//! TODO

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
