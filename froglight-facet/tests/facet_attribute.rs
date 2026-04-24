//! TODO
#![no_std]

use facet::{Facet, Partial, ReflectError};
use froglight_facet::{
    self as mc,
    facet::FacetTemplate,
    format::{Reader, Writer, WriterError, serialize::SerializeItem},
};

#[derive(Facet)]
struct Templated {
    #[facet(mc::with = U32Template::WITH)]
    field: u32,
}

struct U32Template;

impl FacetTemplate for U32Template {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let val = item.peek().get::<u32>().unwrap();
        writer.write_bytes(&val.to_be_bytes())
    }

    fn deserialize(
        _partial: Partial<'static, false>,
        _reader: &mut Reader<'_>,
    ) -> Result<Partial<'static, false>, ReflectError> {
        todo!()
    }
}
