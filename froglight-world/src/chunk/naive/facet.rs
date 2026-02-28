use facet::{Partial, Peek};
use facet_minecraft::{
    DeserializeFn, SerializeFn,
    deserialize::{InputCursor, error::DeserializeValueError},
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};

use super::NaiveChunk;

impl NaiveChunk {
    pub(super) const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    pub(super) const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    fn facet_deserialize<'facet, const BORROW: bool>(
        _partial: &mut Partial<'facet, BORROW>,
        _cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        todo!()
    }

    fn facet_serialize<'mem, 'facet>(
        peek: Peek<'mem, 'facet>,
        _writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        let _value = peek.get::<Self>()?;
        todo!()
    }
}
