//! TODO
#![no_std]

use facet::Facet;
use froglight_facet::{
    self as mc,
    facet::FacetTemplate,
    format::{
        Reader, ReaderError, Writer, WriterError, deserialize::DeserializeItem,
        serialize::SerializeItem,
    },
    to_vec,
};

#[test]
fn variable() {
    #[derive(Facet)]
    struct Variable(#[facet(mc::variable)] u32);

    // Check that `Variable(42)` was serialized correctly.
    let serialized = to_vec(&Variable(42)).unwrap();
    assert_eq!(serialized, [42]);
}

#[test]
fn variable_inner() {
    // Check that `#[facet(mc::variable_inner)]` types pass
    // `#[facet(mc::variable)]` to inner fields.

    #[derive(Facet)]
    struct Outer {
        first: Inner,
        #[facet(mc::variable)]
        second: Ignored,
        #[facet(mc::variable)]
        third: Inner,
    }

    #[derive(Facet)]
    #[facet(mc::variable_inner)]
    struct Inner(u32);

    #[derive(Facet)]
    struct Ignored(u32);

    // Check that `outer` was serialized correctly.
    let outer = Outer { first: Inner(100), second: Ignored(100), third: Inner(100) };
    let serialized = to_vec(&outer).unwrap();
    assert_eq!(serialized, [0, 0, 0, 100, 0, 0, 0, 100, 100]);
}

#[test]
fn template() {
    // Check that `#[facet(mc::with = ...)]` works on types.

    #[derive(Facet)]
    #[facet(mc::with = Templated::WITH)]
    struct Templated(u32);

    impl FacetTemplate for Templated {
        fn serialize(
            item: SerializeItem<'_, '_>,
            writer: &mut Writer<'_>,
        ) -> Result<(), WriterError> {
            let val = item.peek().get::<Templated>()?;
            writer.write_bytes(&val.0.to_le_bytes())
        }

        fn deserialize<'facet, const BORROW: bool>(
            item: DeserializeItem<'facet, BORROW>,
            reader: &mut Reader<'_>,
        ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
            let val = u32::from_le_bytes(*reader.get_array()?);
            item.set(Templated(val))
        }
    }

    // Check that `Templated(42)` was serialized correctly.
    let serialized = to_vec(&Templated(42)).unwrap();
    assert_eq!(serialized, [42, 0, 0, 0]);
}

#[test]
fn template_field() {
    // Check that `#[facet(mc::with = ...)]` works on fields.

    #[derive(Facet)]
    struct Templated(#[facet(mc::with = Templated::WITH)] u32);

    impl FacetTemplate for Templated {
        fn serialize(
            item: SerializeItem<'_, '_>,
            writer: &mut Writer<'_>,
        ) -> Result<(), WriterError> {
            let val = item.peek().get::<u32>()?;
            writer.write_bytes(&val.to_le_bytes())
        }

        fn deserialize<'facet, const BORROW: bool>(
            item: DeserializeItem<'facet, BORROW>,
            reader: &mut Reader<'_>,
        ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
            let val = u32::from_le_bytes(*reader.get_array()?);
            item.set(val)
        }
    }

    // Check that `Templated(42)` was serialized correctly.
    let serialized = to_vec(&Templated(42)).unwrap();
    assert_eq!(serialized, [42, 0, 0, 0]);
}
